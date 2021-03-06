mod ffi;

use std::{
    cell::RefCell,
    cmp,
    ffi::CString,
    io::{self, Write},
    mem, process, ptr,
    sync::{Mutex, MutexGuard, Once},
    thread,
    time::{Duration, Instant},
};

use cfg_if::cfg_if;
use ffi::*;
use libc::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TASSLError {
    #[error("std::ffi::NulError")]
    FFINulError(#[from] std::ffi::NulError),

    #[error("tassl auth error")]
    ServiceError { code: i32, message: String },

    #[error("tassl custom error")]
    CustomError { message: String },
}

pub struct TASSL {
    ctx: RefCell<Option<*mut SSL_CTX>>,
    ssl: RefCell<Option<*mut SSL>>,
    timeout_seconds: i64,
}

impl TASSL {
    fn parse_ffi_invoke_result(&self, code: c_int, message: &str) -> Result<c_int, TASSLError> {
        if code <= 0 {
            Err(TASSLError::ServiceError {
                code,
                message: message.to_owned(),
            })
        } else {
            Ok(code)
        }
    }

    pub fn new(timeout_seconds: i64) -> TASSL {
        TASSL {
            ctx: RefCell::new(None),
            ssl: RefCell::new(None),
            timeout_seconds,
        }
    }

    pub fn init(&self) {
        static mut MUTEXES: *mut Vec<Mutex<()>> = 0 as *mut Vec<Mutex<()>>;
        static mut GUARDS: *mut Vec<Option<MutexGuard<'static, ()>>> =
            0 as *mut Vec<Option<MutexGuard<'static, ()>>>;
        unsafe extern "C" fn locking_function(
            mode: c_int,
            n: c_int,
            _file: *const c_char,
            _line: c_int,
        ) {
            let mutex = &(*MUTEXES)[n as usize];
            if mode & CRYPTO_LOCK != 0 {
                (*GUARDS)[n as usize] = Some(mutex.lock().unwrap());
            } else {
                if let None = (*GUARDS)[n as usize].take() {
                    let _ = writeln!(
                        io::stderr(),
                        "BUG: TASSL lock {} already unlocked, aborting",
                        n
                    );
                    process::abort();
                }
            }
        }
        cfg_if! {
            if #[cfg(unix)] {
                fn set_id_callback() {
                    unsafe extern "C" fn thread_id() -> c_ulong {
                        ::libc::pthread_self() as c_ulong
                    }
                    unsafe {
                        CRYPTO_set_id_callback(thread_id);
                    }
                }
            } else {
                fn set_id_callback() {}
            }
        }
        static INIT: Once = Once::new();
        INIT.call_once(|| unsafe {
            SSL_library_init();
            OPENSSL_add_all_algorithms_noconf();
            SSL_load_error_strings();
            ERR_load_crypto_strings();
            let num_locks = CRYPTO_num_locks();
            let mut mutexes = Box::new(Vec::new());
            for _ in 0..num_locks {
                mutexes.push(Mutex::new(()));
            }
            MUTEXES = mem::transmute(mutexes);
            let guards: Box<Vec<Option<MutexGuard<()>>>> =
                Box::new((0..num_locks).map(|_| None).collect());
            GUARDS = mem::transmute(guards);

            CRYPTO_set_locking_callback(locking_function);
            set_id_callback();
        })
    }

    pub fn load_auth_files(
        &self,
        ca_cert_file: &str,
        sign_key_file: &str,
        sign_cert_file: &str,
        enc_key_file: &str,
        enc_cert_file: &str,
    ) -> Result<(), TASSLError> {
        unsafe {
            if self.ctx.borrow().is_none() {
                let ctx = SSL_CTX_new(TLSv1_2_client_method());
                SSL_CTX_set_timeout(ctx, self.timeout_seconds);
                SSL_CTX_set_mode(ctx, SSL_MODE_AUTO_RETRY);
                SSL_CTX_set_verify(ctx, SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT, None);
                SSL_CTX_set_verify_depth(ctx, 10);
                *self.ctx.borrow_mut() = Some(ctx);
            }
            let ctx = self.ctx.borrow().unwrap();
            self.parse_ffi_invoke_result(
                SSL_CTX_load_verify_locations(
                    ctx,
                    CString::new(ca_cert_file)?.as_ptr() as *const _,
                    ptr::null(),
                ),
                &format!(
                    "SSL_CTX_load_verify_locations invoked failed. Ca Cert File:{:?}",
                    ca_cert_file
                ),
            )?;
            self.parse_ffi_invoke_result(
                SSL_CTX_use_certificate_chain_file(
                    ctx,
                    CString::new(sign_cert_file)?.as_ptr() as *const _,
                ),
                &format!(
                    "SSL_CTX_use_certificate_chain_file invoked failed. Sign Cert File:{:?}",
                    sign_cert_file
                ),
            )?;
            self.parse_ffi_invoke_result(
                SSL_CTX_use_PrivateKey_file(
                    ctx,
                    CString::new(sign_key_file)?.as_ptr() as *const _,
                    SSL_FILETYPE_PEM,
                ),
                &format!(
                    "SSL_CTX_use_PrivateKey_file invoked failed. Sign Key File:{:?}",
                    sign_key_file
                ),
            )?;
            let mut check_enc_private_key = false;
            if enc_cert_file.len() > 0 && enc_key_file.len() > 0 {
                self.parse_ffi_invoke_result(
                    SSL_CTX_use_certificate_file(
                        ctx,
                        CString::new(enc_cert_file)?.as_ptr() as *const _,
                        SSL_FILETYPE_PEM,
                    ),
                    &format!(
                        "SSL_CTX_use_certificate_file invoked failed. Enc Cert File:{:?}",
                        enc_cert_file
                    ),
                )?;
                self.parse_ffi_invoke_result(
                    SSL_CTX_use_enc_PrivateKey_file(
                        ctx,
                        CString::new(enc_key_file)?.as_ptr() as *const _,
                        SSL_FILETYPE_PEM,
                    ),
                    &format!(
                        "SSL_CTX_use_enc_PrivateKey_file invoked failed. Enc Key File:{:?}",
                        enc_key_file
                    ),
                )?;
                check_enc_private_key = true;
            }
            self.parse_ffi_invoke_result(
                SSL_CTX_check_private_key(ctx),
                "SSL_CTX_check_private_key invoked failed",
            )?;
            if check_enc_private_key {
                self.parse_ffi_invoke_result(
                    SSL_CTX_check_enc_private_key(ctx),
                    "SSL_CTX_check_enc_private_key invoked failed",
                )?;
            }
            Ok(())
        }
    }

    pub fn connect(&self, host: &str, port: i32) -> Result<(), TASSLError> {
        unsafe {
            if self.ssl.borrow().is_none() {
                *self.ssl.borrow_mut() = Some(SSL_new(self.ctx.borrow().unwrap()));
            }
            let ssl = self.ssl.borrow().unwrap();
            let connect = BIO_new_connect(CString::new(format!("{}:{}", host, port))?.as_ptr());
            SSL_set_bio(ssl, connect, connect);
            SSL_set_connect_state(ssl);
            let start = Instant::now();
            let timeout_milliseconds = (1000 * self.timeout_seconds) as u128;
            while Instant::now().duration_since(start).as_millis() < timeout_milliseconds {
                if SSL_do_handshake(ssl) <= 0 {
                    thread::sleep(Duration::from_millis(200));
                    continue;
                }
                return Ok(());
            }
            Err(TASSLError::CustomError {
                message: "Error Of SSL do handshake".to_owned(),
            })
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize, TASSLError> {
        if buf.is_empty() {
            return Ok(0);
        }
        let len = cmp::min(c_int::MAX as usize, buf.len()) as c_int;
        unsafe {
            let write_result = self.parse_ffi_invoke_result(
                SSL_write(
                    self.ssl.borrow().unwrap(),
                    buf.as_ptr() as *const c_void,
                    len,
                ),
                "SSL_write invoked failed",
            );
            match write_result {
                Ok(v) => Ok(v as usize),
                Err(error) => {
                    SSL_clear(self.ssl.borrow().unwrap());
                    Err(error)
                }
            }
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, TASSLError> {
        if buf.is_empty() {
            return Ok(0);
        }
        let len = cmp::min(c_int::MAX as usize, buf.len()) as c_int;
        unsafe {
            let read_result = self.parse_ffi_invoke_result(
                SSL_read(self.ssl.borrow().unwrap(), buf.as_ptr() as *mut c_void, len),
                "SSL_read invoked failed",
            );
            match read_result {
                Ok(v) => Ok(v as usize),
                Err(error) => Err(error),
            }
        }
    }

    pub fn close(&self) {
        unsafe {
            let ssl = self.ssl.borrow();
            if ssl.is_some() {
                SSL_shutdown(ssl.unwrap());
            }
        }
    }
}

impl Drop for TASSL {
    fn drop(&mut self) {
        unsafe {
            let ssl = self.ssl.borrow();
            if ssl.is_some() {
                let ssl = ssl.unwrap();
                SSL_shutdown(ssl);
                SSL_free(ssl);
            }
            let ctx = self.ctx.borrow();
            if ctx.is_some() {
                SSL_CTX_free(ctx.unwrap());
            }
        }
    }
}
