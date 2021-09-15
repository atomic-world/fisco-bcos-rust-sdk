use thiserror::Error;
use std::{fs, path::Path};
use keccak_hash::keccak;
use openssl::{ec::{EcKey}};
use wedpr_l_crypto_hash_sm3::WedprSm3;
use wedpr_l_libsm::sm2::signature::SigCtx;
use wedpr_l_crypto_signature_secp256k1::WedprSecp256k1Recover;
use wedpr_l_utils::traits::Hash;

pub struct Account {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),

    #[error("openssl::error::ErrorStack")]
    ErrorStack(#[from] openssl::error::ErrorStack),
}

fn create_sm_account(private_key: &Vec<u8>) -> Result<Account, AccountError> {
    let sig_ctx = SigCtx::new();
    let secret_key = sig_ctx.load_seckey(&private_key).unwrap();
    let derived_public_key = sig_ctx.pk_from_sk(&secret_key);
    let mut public_key = sig_ctx.serialize_pubkey(&derived_public_key, false);
    if public_key.len() == 65 {
        public_key = public_key[1..].to_vec(); // 去掉压缩标记
    }
    let sm3_hash = WedprSm3::default();
    let public_key_hash = sm3_hash.hash(&public_key);
    let address = public_key_hash[12..].to_vec();
    Ok(Account { private_key: private_key.clone(), public_key, address })
}

fn create_ecdsa_account(private_key: &Vec<u8>) -> Result<Account, AccountError> {
    let secp_256k1_recover = WedprSecp256k1Recover::default();
    let mut public_key = secp_256k1_recover.derive_public_key(&private_key).unwrap();
    if public_key.len() == 65 {
        public_key = public_key[1..].to_vec(); // 去掉压缩标记
    }
    let public_key_hash = Vec::from(keccak(&public_key).as_bytes());
    let address = public_key_hash[12..].to_vec();
    Ok(Account { private_key: private_key.clone(), public_key, address })
}

pub fn create_account_from_pem(pem_file_path: &str, sm_crypto: bool) -> Result<Account, AccountError> {
    let pem_data = fs::read(Path::new(pem_file_path))?;
    let private_key = hex::decode(EcKey::private_key_from_pem(&pem_data)?.private_key().to_hex_str()?.to_string())?;
    if sm_crypto {
        create_sm_account(&private_key)
    } else {
        create_ecdsa_account(&private_key)
    }
}