use thiserror::Error;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

use crate::abi::ABIError;
use crate::web3::service_error::ServiceError as Web3ServiceError;

#[derive(Error, Debug)]
pub enum PrecompiledServiceError {
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
const TABLE_NAME_AND_ADDRESS_EXIST: i32 = -51000;
const TABLE_NAME_AND_ADDRESS_NOT_EXIST: i32 = -51001;
const INVALID_NODE_ID: i32 = -51100;
const LAST_SEALER: i32 = -51101;
const P2P_NETWORK: i32 = -51102;
const GROUP_PEERS: i32 = -51103;
const SEALER_LIST: i32 = -51104;
const OBSERVER_LIST: i32 = -51105;
const CONTRACT_NAME_AND_VERSION_EXIST: i32 = -51200;
const VERSION_EXCEEDS: i32 = -51201;
const INVALID_KEY: i32 = -51300;

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
        TABLE_NAME_AND_ADDRESS_EXIST => "Table name and address already exist".to_owned(),
        TABLE_NAME_AND_ADDRESS_NOT_EXIST => "Table name and address does not exist".to_owned(),
        INVALID_NODE_ID => "Invalid node ID".to_owned(),
        LAST_SEALER => "The last sealer cannot be removed".to_owned(),
        P2P_NETWORK => "The node is not reachable".to_owned(),
        GROUP_PEERS => "The node is not a group peer".to_owned(),
        SEALER_LIST => "The node is already in the sealer list".to_owned(),
        OBSERVER_LIST => "The node is already in the observer list".to_owned(),
        CONTRACT_NAME_AND_VERSION_EXIST => "Contract name and version already exist".to_owned(),
        VERSION_EXCEEDS => "Version string length exceeds the maximum limit".to_owned(),
        INVALID_KEY => "Invalid configuration entry".to_owned(),
        _ => format!("unknown output code:{:?}", code)
    };
    Err(PrecompiledServiceError::FiscoBcosError { code, message })
}