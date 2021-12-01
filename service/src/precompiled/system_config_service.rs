use crate::abi::ABI;
use crate::helpers::parse_json_string;
use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{parse_output, PrecompiledServiceError};

const ADDRESS: &str = "0x0000000000000000000000000000000000001000";
const ABI: &str = r#"[{"constant":false,"inputs":[{"name":"key","type":"string"},{"name":"value","type":"string"}],"name":"setValueByKey","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

pub struct SystemConfigService<'a> {
    web3_service: &'a Web3Service,
}

impl SystemConfigService<'_> {
    pub fn new(web3_service: &Web3Service) -> SystemConfigService {
        SystemConfigService {
            web3_service
        }
    }

    pub async fn set_value_by_key(&self, key: &str, value: &str) -> Result<i32, PrecompiledServiceError> {
        let abi_content = Some(Vec::from(ABI.as_bytes()));
        let abi_bin_content: Option<Vec<u8>> = None;
        let abi = ABI::new(
            &abi_content,
            &abi_bin_content,
            "SystemConfigPrecompiled",
            self.web3_service.get_config().sm_crypto,
        )?;
        let params = vec![key.to_owned(), value.to_owned()];
        let tokens = abi.parse_function_tokens("setValueByKey", &params)?;
        let transaction_hash = self.web3_service.send_transaction_with_abi(
            "sendRawTransaction",
            ADDRESS,
            &abi,
            "setValueByKey",
            &tokens
        ).await?;
        let transaction_receipt= self.web3_service.get_transaction_receipt_with_timeout(&transaction_hash).await?;
        if transaction_receipt.is_null() {
            return Err(PrecompiledServiceError::CustomError {
                message: format!(
                    "Transaction c invoked, but the action for fetching transaction receipt is timeout. Transaction hash is {:?}",
                    transaction_hash
                ),
            });
        }
        parse_output(&parse_json_string(&transaction_receipt["output"]))
    }
}