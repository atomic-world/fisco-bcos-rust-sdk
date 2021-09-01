use thiserror::Error;
use keccak_hash::keccak;
use std::{fs, path::Path};
use openssl::{ ec::{EcKey} };
use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub struct Account {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("hyper::Error")]
    Secp256k1Error(#[from] secp256k1::Error),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),

    #[error("openssl::error::ErrorStack")]
    ErrorStack(#[from] openssl::error::ErrorStack),
}

pub fn create_account_from_pem(pem_file_path: &str) -> Result<Account, AccountError> {
    let pem_data = fs::read(Path::new(pem_file_path))?;
    let private_key = hex::decode(EcKey::private_key_from_pem(&pem_data)?.private_key().to_hex_str()?.to_string())?;
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key).serialize_uncompressed().to_vec();
    let public_key_hash = if public_key.len() == 65 {
        Vec::from(keccak(&public_key[1..]).as_bytes()) // 去掉压缩标记
    } else {
        Vec::from(keccak(&public_key).as_bytes())
    };
    let address = public_key_hash[12..].to_vec();
    Ok(Account { private_key, public_key, address })
}