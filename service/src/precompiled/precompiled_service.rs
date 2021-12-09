use ethabi::{Int, Token};
use thiserror::Error;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use byte_slice_cast::AsByteSlice;
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

// ChainGovernancePrecompiled
const CURRENT_VALUE_IS_EXPECTED_VALUE: i32 = -52012;
const ACCOUNT_FROZEN: i32 = -52011;
const ACCOUNT_ALREADY_AVAILABLE: i32 = -52010;
const INVALID_ACCOUNT_ADDRESS: i32 = -52009;
const ACCOUNT_NOT_EXIST: i32 = -52008;
const OPERATOR_NOT_EXIST: i32 = -52007;
const OPERATOR_EXIST: i32 = -52006;
const COMMITTEE_MEMBER_CANNOT_BE_OPERATOR: i32 = -52005;
const OPERATOR_CANNOT_BE_COMMITTEE_MEMBER: i32 = -52004;
const INVALID_THRESHOLD: i32 = -52003;
const INVALID_REQUEST_PERMISSION_DENIED: i32 = -52002;
const COMMITTEE_MEMBER_NOT_EXIST: i32 = -52001;
const COMMITTEE_MEMBER_EXIST: i32 = -52000;

// ContractLifeCyclePrecompiled
const INVALID_REVOKE_LAST_AUTHORIZATION: i32 = -51907;
const INVALID_NON_EXIST_AUTHORIZATION: i32 = -51906;
const INVALID_NO_AUTHORIZED: i32 = -51905;
const INVALID_TABLE_NOT_EXIST: i32 = -51904;
const INVALID_CONTRACT_ADDRESS: i32 = -51903;
const INVALID_CONTRACT_REPEAT_AUTHORIZATION: i32 = -51902;
const INVALID_CONTRACT_AVAILABLE: i32 = -51901;
const INVALID_CONTRACT_FROZEN: i32 = -51900;

// RingSigPrecompiled
const VERIFY_RING_SIG_FAILED: i32 = -51800;

// GroupSigPrecompiled
const VERIFY_GROUP_SIG_FAILED: i32 = -51700;

// PaillierPrecompiled
const INVALID_CIPHERS: i32 = -51600;

// CRUDPrecompiled
const INVALID_UPDATE_TABLE_KEY: i32 = -51503;
const CONDITION_OPERATION_UNDEFINED: i32 = -51502;
const PARSE_CONDITION_ERROR: i32 = -51501;
const PARSE_ENTRY_ERROR: i32 = -51500;

// SystemConfigPrecompiled
const INVALID_CONFIGURATION_VALUES: i32 = -51300;

// CNSPrecompiled
const VERSION_LENGTH_OVERFLOW: i32 = -51201;
const ADDRESS_AND_VERSION_EXIST: i32 = -51200;

// ConsensusPrecompiled
const LAST_SEALER: i32 = -51101;
const INVALID_NODE_ID: i32 = -51100;

// PermissionPrecompiled
const COMMITTEE_PERMISSION: i32 = -51004;
const CONTRACT_NOT_EXIST: i32 = -51003;
const TABLE_NAME_OVERFLOW: i32 = -51002;
const TABLE_AND_ADDRESS_NOT_EXIST: i32 = -51001;
const TABLE_AND_ADDRESS_EXIST: i32 = -51000;

// Common error code among all precompiled contracts
const ADDRESS_INVALID: i32 = -50102;
const UNKNOWN_FUNCTION_CALL: i32 = -50101;
const TABLE_NOT_EXIST: i32 = -50100;

const NO_AUTHORIZED: i32 = -50000;
const TABLE_NAME_ALREADY_EXIST: i32 = -50001;
const TABLE_NAME_LENGTH_OVERFLOW: i32 = -50002;
const TABLE_FILED_LENGTH_OVERFLOW: i32 = -50003;
const TABLE_FILED_TOTAL_LENGTH_OVERFLOW: i32 = -50004;
const TABLE_KEY_VALUE_LENGTH_OVERFLOW: i32 = -50005;
const TABLE_FIELD_VALUE_LENGTH_OVERFLOW: i32 = -50006;
const TABLE_DUPLICATE_FIELD: i32 = -50007;
const TABLE_INVALIDATE_FIELD: i32 = -50008;

pub(crate) fn parse_output(output: &Int) -> Result<i32, PrecompiledServiceError> {
    let bytes = output.as_byte_slice();
    let bit_number = BigInt::from_signed_bytes_le(bytes);
    let code = bit_number.to_i32().unwrap_or(-1);
    if code >= 0 {
      return Ok(code);
    }

    let message = match code {
        CURRENT_VALUE_IS_EXPECTED_VALUE => "The current value is expected".to_owned(),
        ACCOUNT_FROZEN => "The account is frozen".to_owned(),
        ACCOUNT_ALREADY_AVAILABLE => "The account is already available".to_owned(),
        INVALID_ACCOUNT_ADDRESS => "Invalid account address".to_owned(),
        ACCOUNT_NOT_EXIST => "Account not exist, you can create a blockchain account by using this account to deploy contracts on the chain".to_owned(),
        OPERATOR_NOT_EXIST => "The operator not exist".to_owned(),
        OPERATOR_EXIST => "The operator already exist".to_owned(),
        COMMITTEE_MEMBER_CANNOT_BE_OPERATOR => "The committee member cannot be operator".to_owned(),
        OPERATOR_CANNOT_BE_COMMITTEE_MEMBER => "The operator or cnsManager cannot be committee member".to_owned(),
        INVALID_THRESHOLD => "Invalid threshold, threshold should from 0 to 99".to_owned(),
        INVALID_REQUEST_PERMISSION_DENIED => "Invalid request for permission deny".to_owned(),
        COMMITTEE_MEMBER_NOT_EXIST => "The committee member not exist".to_owned(),
        COMMITTEE_MEMBER_EXIST => "The committee member already exist".to_owned(),
        INVALID_REVOKE_LAST_AUTHORIZATION => "There is only one contract status manager left, and the revoke operation cannot be performed".to_owned(),
        INVALID_NON_EXIST_AUTHORIZATION => "The contract status manager doesn't exist".to_owned(),
        INVALID_NO_AUTHORIZED => "Have no permission to access the contract table".to_owned(),
        INVALID_TABLE_NOT_EXIST => "The queried contract address doesn't exist".to_owned(),
        INVALID_CONTRACT_ADDRESS => "The contract address is invalid".to_owned(),
        INVALID_CONTRACT_REPEAT_AUTHORIZATION => "The contract has been granted authorization with same user".to_owned(),
        INVALID_CONTRACT_AVAILABLE => "The contract is available".to_owned(),
        INVALID_CONTRACT_FROZEN => "The contract has been frozen".to_owned(),
        VERIFY_RING_SIG_FAILED => "Verify ring signature failed".to_owned(),
        VERIFY_GROUP_SIG_FAILED => "Verify group signature failed".to_owned(),
        INVALID_CIPHERS => "Execute PaillierAdd failed".to_owned(),
        INVALID_UPDATE_TABLE_KEY => "Don't update the table key".to_owned(),
        CONDITION_OPERATION_UNDEFINED => "Undefined function of Condition Precompiled".to_owned(),
        PARSE_CONDITION_ERROR => "Parse the input of Condition Precompiled failed".to_owned(),
        PARSE_ENTRY_ERROR => "Parse the input of the Entry Precompiled failed".to_owned(),
        INVALID_CONFIGURATION_VALUES => "Invalid configuration entry".to_owned(),
        VERSION_LENGTH_OVERFLOW => "The version string length exceeds the maximum limit".to_owned(),
        ADDRESS_AND_VERSION_EXIST => "The contract name and version already exist".to_owned(),
        LAST_SEALER => "The last sealer cannot be removed".to_owned(),
        INVALID_NODE_ID => "Invalid node ID".to_owned(),
        COMMITTEE_PERMISSION => "The committee permission control by ChainGovernancePrecompiled are recommended".to_owned(),
        CONTRACT_NOT_EXIST => "The contract doesn't exist".to_owned(),
        TABLE_NAME_OVERFLOW => "The table name string length exceeds the maximum limit".to_owned(),
        TABLE_AND_ADDRESS_NOT_EXIST => "The table name and address not exist".to_owned(),
        TABLE_AND_ADDRESS_EXIST => "The table name and address already exist".to_owned(),
        ADDRESS_INVALID => "Invalid address format".to_owned(),
        UNKNOWN_FUNCTION_CALL => "Undefined function".to_owned(),
        TABLE_NOT_EXIST => "Open table failed, please check the existence of the table".to_owned(),
        NO_AUTHORIZED => "Permission denied".to_owned(),
        TABLE_NAME_ALREADY_EXIST => "The table already exist".to_owned(),
        TABLE_NAME_LENGTH_OVERFLOW => "The table name length exceeds the limit 48".to_owned(),
        TABLE_FILED_LENGTH_OVERFLOW => "The table field name exceeds the limit 64".to_owned(),
        TABLE_FILED_TOTAL_LENGTH_OVERFLOW => "The length of all the fields name exceeds the limit 1024".to_owned(),
        TABLE_KEY_VALUE_LENGTH_OVERFLOW => "The value exceeds the limit, key max length is 255, field value max length is 1024".to_owned(),
        TABLE_FIELD_VALUE_LENGTH_OVERFLOW => "The field value exceeds the limit 1024".to_owned(),
        TABLE_DUPLICATE_FIELD => "The table contains duplicated field".to_owned(),
        TABLE_INVALIDATE_FIELD => "Invalid table name or field name".to_owned(),
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
    let tokens = abi.decode_output(
        method,
        &parse_json_string(&transaction_receipt["output"]),
    )?.unwrap();
    let output = tokens[0].clone().into_int().unwrap();
    parse_output(&output)
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