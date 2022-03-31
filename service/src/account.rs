use std::{fs, path::Path};

use keccak_hash::keccak;
use thiserror::Error;
use wedpr_l_crypto_hash_sm3::WedprSm3;
use wedpr_l_crypto_signature_secp256k1::WedprSecp256k1Recover;
use wedpr_l_libsm::sm2::signature::SigCtx;
use wedpr_l_utils::traits::Hash;

pub struct Account {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("pem::PemError")]
    PemError(#[from] pem::PemError),

    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),

    #[error("account custom error")]
    CustomError { message: String },
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
    Ok(Account {
        private_key: private_key.clone(),
        public_key,
        address,
    })
}

fn create_ecdsa_account(private_key: &Vec<u8>) -> Result<Account, AccountError> {
    let secp_256k1_recover = WedprSecp256k1Recover::default();
    let mut public_key = secp_256k1_recover.derive_public_key(&private_key).unwrap();
    if public_key.len() == 65 {
        public_key = public_key[1..].to_vec(); // 去掉压缩标记
    }
    let public_key_hash = Vec::from(keccak(&public_key).as_bytes());
    let address = public_key_hash[12..].to_vec();
    Ok(Account {
        private_key: private_key.clone(),
        public_key,
        address,
    })
}

const EC_PRIVATE_KEY_PREFIX: &str = "30740201010420";
const PRIVATE_KEY_PREFIX_SM: &str =
    "308187020100301306072a8648ce3d020106082a811ccf5501822d046d306b0201010420";
const PRIVATE_KEY_PREFIX_LEN: usize = 66;

fn get_private_key(pem_file_path: &str, sm_crypto: bool) -> Result<Vec<u8>, AccountError> {
    let private_key = pem::parse(fs::read(Path::new(pem_file_path))?)?.contents;
    let private_key_hex = hex::encode(&private_key);
    if sm_crypto {
        if private_key_hex.starts_with(PRIVATE_KEY_PREFIX_SM) {
            let prefix_len = PRIVATE_KEY_PREFIX_SM.len();
            Ok(hex::decode(&private_key_hex[prefix_len..prefix_len + 64])?)
        } else {
            Err(AccountError::CustomError {
                message: "expected `EC PRIVATE KEY` or `PRIVATE KEY`".to_owned(),
            })
        }
    } else {
        let prefix_len = if private_key_hex.starts_with(EC_PRIVATE_KEY_PREFIX) {
            EC_PRIVATE_KEY_PREFIX.len()
        } else {
            PRIVATE_KEY_PREFIX_LEN
        };
        Ok(hex::decode(&private_key_hex[prefix_len..prefix_len + 64])?)
    }
}

pub fn create_account_from_pem(
    pem_file_path: &str,
    sm_crypto: bool,
) -> Result<Account, AccountError> {
    let private_key = get_private_key(pem_file_path, sm_crypto)?;
    if sm_crypto {
        create_sm_account(&private_key)
    } else {
        create_ecdsa_account(&private_key)
    }
}
