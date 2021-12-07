use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{PrecompiledServiceError, send_transaction};

const ADDRESS: &str = "0x0000000000000000000000000000000000001000";
const ABI_CONTENT: &str = r#"[{"constant":false,"inputs":[{"name":"key","type":"string"},{"name":"value","type":"string"}],"name":"setValueByKey","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

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
        let params = vec![key.to_owned(), value.to_owned()];
        send_transaction(
            self.web3_service,
            "SystemConfigPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "setValueByKey",
            &params,
        ).await
    }
}