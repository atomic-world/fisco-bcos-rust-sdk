#![allow(non_snake_case, non_camel_case_types)]

use std::ptr;

use cfg_if::cfg_if;
use libc::*;

pub(crate) type tls_session_ticket_ext_cb_fn =
    Option<unsafe extern "C" fn(*mut SSL, *const c_uchar, c_int, *mut c_void) -> c_int>;

pub(crate) type tls_session_secret_cb_fn = Option<
    unsafe extern "C" fn(
        *mut SSL,
        *mut c_void,
        *mut c_int,
        *mut stack_st_SSL_CIPHER,
        *mut *mut SSL_CIPHER,
        *mut c_void,
    ) -> c_int,
>;

pub(crate) type GEN_SESSION_CB =
    Option<unsafe extern "C" fn(*const SSL, *mut c_uchar, *mut c_uint) -> c_int>;

pub(crate) type bio_info_cb =
    Option<unsafe extern "C" fn(*mut BIO, c_int, *const c_char, c_int, c_long, c_long)>;

pub(crate) enum ASN1_OBJECT {}

pub(crate) enum ASN1_TIME {}

pub(crate) enum ASN1_BIT_STRING {}

pub(crate) enum ENGINE {}

pub(crate) enum EVP_MD {}

pub(crate) enum EVP_PKEY_CTX {}

pub(crate) enum EVP_CIPHER_CTX {}

pub(crate) enum SSL_CIPHER {}

pub(crate) enum SSL_METHOD {}

pub(crate) enum X509_STORE_CTX {}

pub(crate) const CRYPTO_LOCK: c_int = 1;

pub(crate) const X509_FILETYPE_PEM: c_int = 1;

pub(crate) const SSL_FILETYPE_PEM: c_int = X509_FILETYPE_PEM;

pub(crate) const SSL_CTRL_MODE: c_int = 33;

pub(crate) const SSL_MODE_AUTO_RETRY: c_long = 0x4;

pub(crate) const SSL_VERIFY_PEER: c_int = 1;

pub(crate) const SSL_VERIFY_FAIL_IF_NO_PEER_CERT: c_int = 2;

#[cfg(not(ossl110))]
pub(crate) const SSL_MAX_SID_CTX_LENGTH: c_int = 32;

#[cfg(not(ossl110))]
pub(crate) const SSL_MAX_SSL_SESSION_ID_LENGTH: c_int = 32;

#[cfg(not(ossl110))]
pub(crate) const SSL_MAX_KEY_ARG_LENGTH: c_int = 8;

cfg_if! {
    if #[cfg(ossl110)] {
      pub(crate) enum OPENSSL_STACK {}
    } else {
        #[repr(C)]
        pub(crate) struct _STACK {
           pub(crate) num: c_int,
           pub(crate) data: *mut *mut c_char,
           pub(crate) sorted: c_int,
           pub(crate) num_alloc: c_int,
           pub(crate) comp: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        }
    }
}

macro_rules! stack {
    ($t:ident) => {
        cfg_if! {
            if #[cfg(ossl110)] {
               pub(crate) enum $t {}
            } else {
                #[repr(C)]
               pub(crate) struct $t {
                   pub(crate) stack: _STACK,
                }
            }
        }
    };
}

stack!(stack_st_void);
stack!(stack_st_SSL_CIPHER);
stack!(stack_st_X509_NAME);
stack!(stack_st_X509_EXTENSION);

cfg_if! {
    if #[cfg(ossl110)] {
       pub(crate) enum CRYPTO_EX_DATA {}
    } else {
        #[repr(C)]
        pub(crate) struct CRYPTO_EX_DATA {
            pub(crate) sk: *mut stack_st_void,
            pub(crate) dummy: c_int,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
       pub(crate) enum SSL_CTX {}
    } else {
        #[repr(C)]
        pub(crate) struct SSL_CTX {
            method: *mut c_void,
            cipher_list: *mut c_void,
            cipher_list_by_id: *mut c_void,
            cert_store: *mut c_void,
            sessions: *mut c_void,
            session_cache_size: c_ulong,
            session_cache_head: *mut c_void,
            session_cache_tail: *mut c_void,
            session_cache_mode: c_int,
            session_timeout: c_long,
            new_session_cb: *mut c_void,
            remove_session_cb: *mut c_void,
            get_session_cb: *mut c_void,
            stats: [c_int; 11],
            pub(crate) references: c_int,
            app_verify_callback: *mut c_void,
            app_verify_arg: *mut c_void,
            default_passwd_callback: *mut c_void,
            default_passwd_callback_userdata: *mut c_void,
            client_cert_cb: *mut c_void,
            app_gen_cookie_cb: *mut c_void,
            app_verify_cookie_cb: *mut c_void,
            ex_dat: CRYPTO_EX_DATA,
            rsa_md5: *mut c_void,
            md5: *mut c_void,
            sha1: *mut c_void,
            extra_certs: *mut c_void,
            comp_methods: *mut c_void,
            info_callback: *mut c_void,
            client_CA: *mut c_void,
            options: c_ulong,
            mode: c_ulong,
            max_cert_list: c_long,
            cert: *mut c_void,
            read_ahead: c_int,
            msg_callback: *mut c_void,
            msg_callback_arg: *mut c_void,
            verify_mode: c_int,
            sid_ctx_length: c_uint,
            sid_ctx: [c_uchar; 32],
            default_verify_callback: *mut c_void,
            generate_session_id: *mut c_void,
            param: *mut c_void,
            quiet_shutdown: c_int,
            max_send_fragment: c_uint,

            #[cfg(not(osslconf = "OPENSSL_NO_ENGINE"))]
            client_cert_engine: *mut c_void,

            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_servername_callback: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsect_servername_arg: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick_key_name: [c_uchar; 16],
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick_hmac_key: [c_uchar; 16],
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick_aes_key: [c_uchar; 16],
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ticket_key_cb: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_status_cb: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_status_arg: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_opaque_prf_input_callback: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_opaque_prf_input_callback_arg: *mut c_void,

            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_identity_hint: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_client_callback: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_server_callback: *mut c_void,

            #[cfg(not(osslconf = "OPENSSL_NO_BUF_FREELISTS"))]
            freelist_max_len: c_uint,
            #[cfg(not(osslconf = "OPENSSL_NO_BUF_FREELISTS"))]
            wbuf_freelist: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_BUF_FREELISTS"))]
            rbuf_freelist: *mut c_void,

            #[cfg(not(osslconf = "OPENSSL_NO_SRP"))]
            srp_ctx: SRP_CTX,

            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_NEXTPROTONEG")
            ))]
            next_protos_advertised_cb: *mut c_void,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_NEXTPROTONEG")
            ))]
            next_protos_advertised_cb_arg: *mut c_void,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_NEXTPROTONEG")
            ))]
            next_proto_select_cb: *mut c_void,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_NEXTPROTONEG")
            ))]
            next_proto_select_cb_arg: *mut c_void,

            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl101))]
            srtp_profiles: *mut c_void,
            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl102))]
            alpn_select_cb: *mut c_void,
            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl102))]
            alpn_select_cb_arg: *mut c_void,
            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl102))]
            alpn_client_proto_list: *mut c_void,
            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl102))]
            alpn_client_proto_list_len: c_uint,

            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC"),
                ossl102
            ))]
            tlsext_ecpointformatlist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC"),
                ossl102
            ))]
            tlsext_ecpointformatlist: *mut c_uchar,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC"),
                ossl102
            ))]
            tlsext_ellipticcurvelist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC"),
                ossl102
            ))]
            tlsext_ellipticcurvelist: *mut c_uchar,
        }

        #[repr(C)]
        #[cfg(not(osslconf = "OPENSSL_NO_SRP"))]
        pub(crate) struct SRP_CTX {
            SRP_cb_arg: *mut c_void,
            TLS_ext_srp_username_callback: *mut c_void,
            SRP_verify_param_callback: *mut c_void,
            SRP_give_srp_client_pwd_callback: *mut c_void,
            login: *mut c_void,
            N: *mut c_void,
            g: *mut c_void,
            s: *mut c_void,
            B: *mut c_void,
            A: *mut c_void,
            a: *mut c_void,
            b: *mut c_void,
            v: *mut c_void,
            info: *mut c_void,
            stringth: c_int,
            srp_Mask: c_ulong,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum SSL {}
    }  else {
        #[repr(C)]
        pub(crate) struct SSL {
            version: c_int,
            type_: c_int,
            method: *const SSL_METHOD,
            rbio: *mut c_void,
            wbio: *mut c_void,
            bbio: *mut c_void,
            rwstate: c_int,
            in_handshake: c_int,
            handshake_func: Option<unsafe extern "C" fn(*mut SSL) -> c_int>,
            pub(crate) server: c_int,
            new_session: c_int,
            quiet_session: c_int,
            shutdown: c_int,
            state: c_int,
            rstate: c_int,
            init_buf: *mut c_void,
            init_msg: *mut c_void,
            init_num: c_int,
            init_off: c_int,
            packet: *mut c_uchar,
            packet_length: c_uint,
            s2: *mut c_void,
            s3: *mut c_void,
            d1: *mut c_void,
            read_ahead: c_int,
            msg_callback: Option<
                unsafe extern "C" fn(c_int, c_int, c_int, *const c_void, size_t, *mut SSL, *mut c_void),
            >,
            msg_callback_arg: *mut c_void,
            hit: c_int,
            param: *mut c_void,
            cipher_list: *mut stack_st_SSL_CIPHER,
            cipher_list_by_id: *mut stack_st_SSL_CIPHER,
            mac_flags: c_int,
            enc_read_ctx: *mut EVP_CIPHER_CTX,
            read_hash: *mut EVP_MD_CTX,
            expand: *mut c_void,
            enc_write_ctx: *mut EVP_CIPHER_CTX,
            write_hash: *mut EVP_MD_CTX,
            compress: *mut c_void,
            cert: *mut c_void,
            sid_ctx_length: c_uint,
            sid_ctx: [c_uchar; SSL_MAX_SID_CTX_LENGTH as usize],
            session: *mut SSL_SESSION,
            generate_session_id: GEN_SESSION_CB,
            verify_mode: c_int,
            verify_callback: Option<unsafe extern "C" fn(c_int, *mut X509_STORE_CTX) -> c_int>,
            info_callback: Option<unsafe extern "C" fn(*mut SSL, c_int, c_int)>,
            error: c_int,
            error_code: c_int,
            #[cfg(not(osslconf = "OPENSSL_NO_KRB5"))]
            kssl_ctx: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_client_callback: Option<
                unsafe extern "C" fn(*mut SSL, *const c_char, *mut c_char, c_uint, *mut c_uchar, c_uint)
                    -> c_uint,
            >,
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_server_callback:
                Option<unsafe extern "C" fn(*mut SSL, *const c_char, *mut c_uchar, c_uint) -> c_uint>,
            ctx: *mut SSL_CTX,
            debug: c_int,
            verify_result: c_long,
            ex_data: CRYPTO_EX_DATA,
            client_CA: *mut stack_st_X509_NAME,
            references: c_int,
            options: c_ulong,
            mode: c_ulong,
            max_cert_list: c_long,
            first_packet: c_int,
            client_version: c_int,
            max_send_fragment: c_uint,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_debug_cb:
                Option<unsafe extern "C" fn(*mut SSL, c_int, c_int, *mut c_uchar, c_int, *mut c_void)>,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_debug_arg: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_hostname: *mut c_char,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            servername_done: c_int,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_status_type: c_int,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_status_expected: c_int,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ocsp_ids: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ocsp_exts: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ocsp_resp: *mut c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ocsp_resplen: c_int,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ticket_expected: c_int,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ecpointformatlist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ecpointformatlist: *mut c_uchar,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ellipticcurvelist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ellipticcurvelist: *mut c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_opaque_prf_input: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_opaque_prf_input_len: size_t,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_session_ticket: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_session_ticket_ext_cb: tls_session_ticket_ext_cb_fn,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tls_session_ticket_ext_cb_arg: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tls_session_secret_cb: tls_session_secret_cb_fn,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tls_session_secret_cb_arg: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            initial_ctx: *mut SSL_CTX,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_NEXTPROTONEG")
            ))]
            next_proto_negotiated: *mut c_uchar,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_NEXTPROTONEG")
            ))]
            next_proto_negotiated_len: c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            srtp_profiles: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            srtp_profile: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_heartbeat: c_uint,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_hb_pending: c_uint,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_hb_seq: c_uint,
            renegotiate: c_int,
            #[cfg(not(osslconf = "OPENSSL_NO_SRP"))]
            srp_ctx: SRP_CTX,
            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl102))]
            alpn_client_proto_list: *mut c_uchar,
            #[cfg(all(not(osslconf = "OPENSSL_NO_TLSEXT"), ossl102))]
            alpn_client_proto_list_len: c_uint,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum X509_CINF {}
    } else {
        #[repr(C)]
        pub(crate) struct X509_CINF {
            version: *mut c_void,
            serialNumber: *mut c_void,
            signature: *mut c_void,
            issuer: *mut c_void,
            pub validity: *mut X509_VAL,
            subject: *mut c_void,
            key: *mut c_void,
            issuerUID: *mut c_void,
            subjectUID: *mut c_void,
            pub extensions: *mut stack_st_X509_EXTENSION,
            enc: ASN1_ENCODING,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum X509 {}
    } else {
        #[repr(C)]
        pub(crate) struct X509 {
            pub(crate) cert_info: *mut X509_CINF,
            pub(crate) sig_alg: *mut X509_ALGOR,
            pub(crate) signature: *mut ASN1_BIT_STRING,
            pub(crate) valid: c_int,
            pub(crate) references: c_int,
            pub(crate) name: *mut c_char,
            pub(crate) ex_data: CRYPTO_EX_DATA,
            pub(crate) ex_pathlen: c_long,
            pub(crate) ex_pcpathlen: c_long,
            pub(crate) ex_flags: c_ulong,
            pub(crate) ex_kusage: c_ulong,
            pub(crate) ex_xkusage: c_ulong,
            pub(crate) ex_nscert: c_ulong,
            skid: *mut c_void,
            akid: *mut c_void,
            policy_cache: *mut c_void,
            crldp: *mut c_void,
            altname: *mut c_void,
            nc: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_RFC3779"))]
            rfc3779_addr: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_RFC3779"))]
            rfc3779_asid: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_SHA"))]
            sha1_hash: [c_uchar; 20],
            aux: *mut c_void,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum SSL_SESSION {}
    } else {
        #[repr(C)]
        pub(crate) struct SSL_SESSION {
            ssl_version: c_int,
            key_arg_length: c_uint,
            key_arg: [c_uchar; SSL_MAX_KEY_ARG_LENGTH as usize],
            pub(crate) master_key_length: c_int,
            pub(crate) master_key: [c_uchar; 48],
            session_id_length: c_uint,
            session_id: [c_uchar; SSL_MAX_SSL_SESSION_ID_LENGTH as usize],
            sid_ctx_length: c_uint,
            sid_ctx: [c_uchar; SSL_MAX_SID_CTX_LENGTH as usize],
            #[cfg(not(osslconf = "OPENSSL_NO_KRB5"))]
            krb5_client_princ_len: c_uint,
            #[cfg(not(osslconf = "OPENSSL_NO_KRB5"))]
            krb5_client_princ: [c_uchar; SSL_MAX_KRB5_PRINCIPAL_LENGTH as usize],
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_identity_hint: *mut c_char,
            #[cfg(not(osslconf = "OPENSSL_NO_PSK"))]
            psk_identity: *mut c_char,
            not_resumable: c_int,
            sess_cert: *mut c_void,
            peer: *mut X509,
            verify_result: c_long,
            pub(crate) references: c_int,
            timeout: c_long,
            time: c_long,
            compress_meth: c_uint,
            cipher: *const c_void,
            cipher_id: c_ulong,
            ciphers: *mut c_void,
            ex_data: CRYPTO_EX_DATA,
            prev: *mut c_void,
            next: *mut c_void,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_hostname: *mut c_char,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ecpointformatlist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ecpointformatlist: *mut c_uchar,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ellipticcurvelist_length: size_t,
            #[cfg(all(
                not(osslconf = "OPENSSL_NO_TLSEXT"),
                not(osslconf = "OPENSSL_NO_EC")
            ))]
            tlsext_ellipticcurvelist: *mut c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick: *mut c_uchar,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_ticklen: size_t,
            #[cfg(not(osslconf = "OPENSSL_NO_TLSEXT"))]
            tlsext_tick_lifetime_hint: c_long,
            #[cfg(not(osslconf = "OPENSSL_NO_SRP"))]
            srp_username: *mut c_char,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum EVP_MD_CTX {}
    } else {
        #[repr(C)]
        pub(crate) struct EVP_MD_CTX {
            digest: *mut EVP_MD,
            engine: *mut ENGINE,
            flags: c_ulong,
            md_data: *mut c_void,
            pctx: *mut EVP_PKEY_CTX,
            update: *mut c_void,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum X509_ALGOR {}
    } else {
        #[repr(C)]
        pub(crate) struct X509_ALGOR {
            pub(crate) algorithm: *mut ASN1_OBJECT,
            parameter: *mut c_void,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum BIO {}
    } else {
        #[repr(C)]
        pub(crate) struct BIO {
            pub(crate) method: *mut BIO_METHOD,
            pub(crate) callback: Option<
                unsafe extern "C" fn(*mut BIO, c_int, *const c_char, c_int, c_long, c_long) -> c_long,
            >,
            pub(crate) cb_arg: *mut c_char,
            pub(crate) init: c_int,
            pub(crate) shutdown: c_int,
            pub(crate) flags: c_int,
            pub(crate) retry_reason: c_int,
            pub(crate) num: c_int,
            pub(crate) ptr: *mut c_void,
            pub(crate) next_bio: *mut BIO,
            pub(crate) prev_bio: *mut BIO,
            pub(crate) references: c_int,
            pub(crate) num_read: c_ulong,
            pub(crate) num_write: c_ulong,
            pub(crate) ex_data: CRYPTO_EX_DATA,
        }
    }
}

cfg_if! {
    if #[cfg(ossl110)] {
        pub(crate) enum BIO_METHOD {}
    } else {
        #[repr(C)]
        pub(crate) struct BIO_METHOD {
            pub(crate) type_: c_int,
            pub(crate) name: *const c_char,
            pub(crate) bwrite: Option<unsafe extern "C" fn(*mut BIO, *const c_char, c_int) -> c_int>,
            pub(crate) bread: Option<unsafe extern "C" fn(*mut BIO, *mut c_char, c_int) -> c_int>,
            pub(crate) bputs: Option<unsafe extern "C" fn(*mut BIO, *const c_char) -> c_int>,
            pub(crate) bgets: Option<unsafe extern "C" fn(*mut BIO, *mut c_char, c_int) -> c_int>,
            pub(crate) ctrl: Option<unsafe extern "C" fn(*mut BIO, c_int, c_long, *mut c_void) -> c_long>,
            pub(crate) create: Option<unsafe extern "C" fn(*mut BIO) -> c_int>,
            pub(crate) destroy: Option<unsafe extern "C" fn(*mut BIO) -> c_int>,
            pub(crate) callback_ctrl: Option<unsafe extern "C" fn(*mut BIO, c_int, bio_info_cb) -> c_long>,
        }
    }
}

#[repr(C)]
pub(crate) struct X509_VAL {
    pub(crate) notBefore: *mut ASN1_TIME,
    pub(crate) notAfter: *mut ASN1_TIME,
}

#[repr(C)]
pub(crate) struct ASN1_ENCODING {
    pub(crate) enc: *mut c_uchar,
    pub(crate) len: c_long,
    pub(crate) modified: c_int,
}

extern "C" {
    pub(crate) fn CRYPTO_num_locks() -> c_int;
    pub(crate) fn CRYPTO_set_locking_callback(
        func: unsafe extern "C" fn(mode: c_int, n: c_int, file: *const c_char, line: c_int),
    );
    pub(crate) fn CRYPTO_set_id_callback(func: unsafe extern "C" fn() -> c_ulong);

    pub(crate) fn SSL_library_init() -> c_int;
    pub(crate) fn OPENSSL_add_all_algorithms_noconf();
    pub(crate) fn SSL_load_error_strings();
    pub(crate) fn ERR_load_crypto_strings();

    pub(crate) fn TLSv1_2_client_method() -> *const SSL_METHOD;

    pub(crate) fn SSL_CTX_new(method: *const SSL_METHOD) -> *mut SSL_CTX;
    pub(crate) fn SSL_CTX_set_timeout(ctx: *mut SSL_CTX, t: c_long) -> c_long;
    pub(crate) fn SSL_CTX_free(ctx: *mut SSL_CTX);
    pub(crate) fn SSL_CTX_ctrl(
        ctx: *mut SSL_CTX,
        cmd: c_int,
        larg: c_long,
        parg: *mut c_void,
    ) -> c_long;

    pub(crate) fn SSL_CTX_load_verify_locations(
        ctx: *mut SSL_CTX,
        ca_file: *const c_char,
        ca_path: *const c_char,
    ) -> c_int;
    pub(crate) fn SSL_CTX_use_certificate_chain_file(
        ctx: *mut SSL_CTX,
        file: *const c_char,
    ) -> c_int;
    pub(crate) fn SSL_CTX_use_PrivateKey_file(
        ctx: *mut SSL_CTX,
        file: *const c_char,
        file_type: c_int,
    ) -> c_int;
    pub(crate) fn SSL_CTX_use_certificate_file(
        ctx: *mut SSL_CTX,
        file: *const c_char,
        file_type: c_int,
    ) -> c_int;
    pub(crate) fn SSL_CTX_use_enc_PrivateKey_file(
        ctx: *mut SSL_CTX,
        file: *const c_char,
        file_type: c_int,
    ) -> c_int;
    pub(crate) fn SSL_CTX_check_private_key(ctx: *mut SSL_CTX) -> c_int;
    pub(crate) fn SSL_CTX_check_enc_private_key(ctx: *mut SSL_CTX) -> c_int;

    pub(crate) fn SSL_CTX_set_verify(
        ctx: *mut SSL_CTX,
        mode: c_int,
        verify_callback: Option<extern "C" fn(c_int, *mut X509_STORE_CTX) -> c_int>,
    );
    pub(crate) fn SSL_CTX_set_verify_depth(ctx: *mut SSL_CTX, depth: c_int);

    pub(crate) fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    pub(crate) fn SSL_shutdown(ssl: *mut SSL) -> c_int;
    pub(crate) fn SSL_free(ssl: *mut SSL);

    pub(crate) fn BIO_new_connect(host_port: *const c_char) -> *mut BIO;
    pub(crate) fn SSL_set_bio(ssl: *mut SSL, read_bio: *mut BIO, write_bio: *mut BIO);
    pub(crate) fn SSL_set_connect_state(ssl: *mut SSL);
    pub(crate) fn SSL_do_handshake(ssl: *mut SSL) -> c_int;
    pub(crate) fn SSL_write(ssl: *mut SSL, buf: *const c_void, num: c_int) -> c_int;
    pub(crate) fn SSL_read(ssl: *mut SSL, buf: *mut c_void, num: c_int) -> c_int;
    pub(crate) fn SSL_clear(ssl: *mut SSL) -> c_int;
}

pub(crate) unsafe fn SSL_CTX_set_mode(ctx: *mut SSL_CTX, op: c_long) -> c_long {
    SSL_CTX_ctrl(ctx, SSL_CTRL_MODE, op, ptr::null_mut())
}
