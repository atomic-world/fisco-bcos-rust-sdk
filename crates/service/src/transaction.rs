use thiserror::Error;
use rlp::RlpStream;
use uuid::Uuid;
use std::convert::TryInto;
use ethereum_types::{H512, H160, H256, U256};
use wedpr_l_utils::traits::{Hash, Signature};
use wedpr_l_crypto_hash_keccak256::WedprKeccak256;
use wedpr_l_crypto_signature_secp256k1::WedprSecp256k1Recover;

use crate::account::{Account, AccountError};
use crate::abi::{ABI, ABIError};

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("transaction abi error")]
    TransactionABIError(#[from] ABIError),

    #[error("transaction account error")]
    TransactionAccountError(#[from] AccountError),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),
}

// 编码规则详见：
// https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#rlp
pub fn get_sign_transaction_data(
    account: &Account,
    abi: &ABI,
    group_id: u32,
    chain_id: u32,
    block_limit: u32,
    to_address: &str,
    function_name: &str,
    params: &Vec<String>,
) -> Result<Vec<u8>, TransactionError> {
    let nonce = U256::from(Uuid::new_v4().to_string().replace("-", "").as_bytes());
    let gas_price = U256::from(300000000);
    let gas = U256::from(300000000);
    let block_limit = U256::from(block_limit);
    let receive_address = H160::from_slice(&hex::decode(to_address.to_owned().trim_start_matches("0x").as_bytes())?);
    let value = U256::from(0);
    let data= abi.encode_input(function_name, params)?;
    let chain_id = U256::from(chain_id);
    let group_id = U256::from(group_id);
    let extra_data= b"".to_vec();
    let mut stream = RlpStream::new();
    stream.begin_list(10);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&gas);
    stream.append(&block_limit);
    stream.append(&receive_address);
    stream.append(&value);
    stream.append(&data);
    stream.append(&chain_id);
    stream.append(&group_id);
    stream.append(&extra_data);
    let transaction_encode_data = stream.out().to_vec();
    let keccak256 = WedprKeccak256::default();
    let msg_hash = keccak256.hash(&transaction_encode_data);
    let tx_hash = H256::from_slice(&msg_hash);
    let signer = WedprSecp256k1Recover::default();
    let signature = signer.sign(
        &account.private_key, &tx_hash.as_bytes().to_vec()
    ).unwrap();
    let r = &signature[0..32];
    let s = &signature[32..64];
    let val = (&signature[64..])[0] as u64;
    let v = if val == 4 {
        4_u64.to_be_bytes().to_vec()
    } else {
        (val + 27).to_be_bytes().to_vec()
    };
    let mut stream = RlpStream::new();
    stream.begin_list(13);
    stream.append(&nonce);
    stream.append(&gas_price);
    stream.append(&gas);
    stream.append(&block_limit);
    stream.append(&receive_address);
    stream.append(&value);
    stream.append(&data);
    stream.append(&chain_id);
    stream.append(&group_id);
    stream.append(&extra_data);
    if v.len() == 8 {
        stream.append(&u64::from_be_bytes(v[0..8].try_into().unwrap()));
    } else {
        stream.append(&H512::from_slice(&v));
    }
    stream.append(&H256::from_slice(r));
    stream.append(&H256::from_slice(s));
    Ok(stream.out().to_vec())
}