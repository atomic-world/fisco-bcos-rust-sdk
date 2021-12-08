use ethabi::Token;
use thiserror::Error;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use serde_json::{json, Value as JSONValue};

use crate::abi::{ABI, ABIError};
use crate::helpers::parse_json_string;
use crate::web3::{service::{Service as Web3Service, CallResponse}, service_error::ServiceError as Web3ServiceError};

#[derive(Error, Debug)]
pub enum PrecompiledServiceError {
    #[error("serde_json::Error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("abi error")]
    ABIError(#[from] ABIError),

    #[error("web3 service error")]
    Web3ServiceError(#[from] Web3ServiceError),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),

    #[error("system config custom error")]
    CustomError {
        message: String,
    },

    #[error("fisco bcos response error")]
    FiscoBcosError {
        code: i32,
        message: String,
    }
}

const PERMISSION_DENIED: i32 = -50000;
const TABLE_EXIST: i32 = -50001;
const TABLE_NAME_AND_ADDRESS_NOT_EXIST: i32 = -51001;
const TABLE_NAME_EXCEEDS: i32 = -51002;
const CONTRACT_NOT_EXIST: i32 = -51003;
const SPECIAL_PERMISSIONS: i32 = -51004;
const TABLE_NAME_AND_ADDRESS_EXIST: i32 = -51009;
const INVALID_NODE_ID: i32 = -51100;
const LAST_SEALER: i32 = -51101;
const P2P_NETWORK: i32 = -51102;
const GROUP_PEERS: i32 = -51103;
const SEALER_LIST: i32 = -51104;
const OBSERVER_LIST: i32 = -51105;
const VERSION_EXCEEDS: i32 = -51200;
const CONTRACT_NAME_AND_VERSION_EXIST: i32 = -51201;
const INVALID_INPUT: i32 = -51300;

pub(crate) fn parse_output(output: &str) -> Result<i32, PrecompiledServiceError> {
    let data = hex::decode(output.to_owned().trim_start_matches("0x").as_bytes())?;
    let bit_number = BigInt::from_signed_bytes_be(&data);
    let code = bit_number.to_i32().unwrap_or(-1);
    if code >= 0 {
      return Ok(code);
    }

    let message = match code {
        PERMISSION_DENIED => "Permission denied".to_owned(),
        TABLE_EXIST => "Table name already exist".to_owned(),
        TABLE_NAME_AND_ADDRESS_NOT_EXIST => "Table name and address does not exist".to_owned(),
        TABLE_NAME_EXCEEDS => "The length of the table name exceeds the maximum limit".to_owned(),
        CONTRACT_NOT_EXIST => "Contract does not exist".to_owned(),
        SPECIAL_PERMISSIONS => "Special permissions for ChainGovernancePrecompiled committee".to_owned(),
        TABLE_NAME_AND_ADDRESS_EXIST => "Table name and address already exist".to_owned(),
        INVALID_NODE_ID => "Invalid node ID".to_owned(),
        LAST_SEALER => "The last sealer cannot be removed".to_owned(),
        P2P_NETWORK => "The node is not reachable".to_owned(),
        GROUP_PEERS => "The node is not a group peer".to_owned(),
        SEALER_LIST => "The node is already in the sealer list".to_owned(),
        OBSERVER_LIST => "The node is already in the observer list".to_owned(),
        VERSION_EXCEEDS => "Version string length exceeds the maximum limit".to_owned(),
        CONTRACT_NAME_AND_VERSION_EXIST => "Contract name and version already exist".to_owned(),
        INVALID_INPUT => "Invalid input".to_owned(),
        _ => format!("unknown output code:{:?}", code)
    };
    Err(PrecompiledServiceError::FiscoBcosError { code, message })
}

pub(crate) async fn send_transaction(
    web3_service: &Web3Service,
    contract_name: &str,
    address: &str,
    abi_content: &str,
    method: &str,
    params: &Vec<String>,
) -> Result<i32, PrecompiledServiceError> {
    let abi_content = Some(Vec::from(abi_content.as_bytes()));
    let abi_bin_content: Option<Vec<u8>> = None;
    let abi = ABI::new(
        &abi_content,
        &abi_bin_content,
        contract_name,
        web3_service.get_config().sm_crypto,
    )?;
    let tokens = abi.parse_function_tokens(method, &params)?;
    let transaction_hash = web3_service.send_transaction_with_abi(
        "sendRawTransaction",
        address,
        &abi,
        method,
        &tokens
    ).await?;
    let transaction_receipt= web3_service.get_transaction_receipt_with_timeout(&transaction_hash).await?;
    if transaction_receipt.is_null() {
        return Err(PrecompiledServiceError::CustomError {
            message: format!(
                "Transaction invoked, but the action for fetching transaction receipt is timeout. Transaction hash is {:?}",
                transaction_hash
            ),
        });
    }
    parse_output(&parse_json_string(&transaction_receipt["output"]))
}

pub(crate) async fn call(
    web3_service: &Web3Service,
    contract_name: &str,
    address: &str,
    abi_content: &str,
    method: &str,
    params: &Vec<String>,
) -> Result<CallResponse, PrecompiledServiceError> {
    let abi_content = Some(Vec::from(abi_content.as_bytes()));
    let abi_bin_content: Option<Vec<u8>> = None;
    let abi = ABI::new(
        &abi_content,
        &abi_bin_content,
        contract_name,
        web3_service.get_config().sm_crypto,
    )?;
    let tokens = abi.parse_function_tokens(method, &params)?;
    Ok(web3_service.call_with_abi(address, &abi, method, &tokens).await?)
}

pub(crate) fn parse_string_token_to_json(tokens: &Option<Vec<Token>>) -> Result<JSONValue, PrecompiledServiceError> {
    Ok(
        match tokens {
            None => json!(null),
            Some(tokens) => {
                if tokens.len() > 0 {
                    let output = tokens[0].clone().into_string().unwrap_or(String::from(""));
                    serde_json::from_str(&output)?
                } else {
                    json!(null)
                }
            }
        }
    )
}