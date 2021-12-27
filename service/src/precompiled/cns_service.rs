use serde_json::Value as JSONValue;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{
    PrecompiledServiceError,
    call,
    send_transaction,
    parse_string_token_to_json,
    parse_address_token_to_string,
};

const ADDRESS: &str = "0x0000000000000000000000000000000000001004";
const ABI_CONTENT: &str = r#"[{"constant":true,"inputs":[{"name":"name","type":"string"}],"name":"selectByName","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"name","type":"string"},{"name":"version","type":"string"}],"name":"selectByNameAndVersion","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"addr","type":"string"},{"name":"abi","type":"string"}],"name":"insert","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"name","type":"string"},{"name":"version","type":"string"}],"name":"getContractAddress","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"}]"#;

pub struct CNSService<'l> {
    web3_service: &'l Web3Service,
}

impl<'l> CNSService<'l> {
    pub fn new(web3_service: &'l Web3Service) -> CNSService<'l> {
        CNSService {
            web3_service
        }
    }

    pub async fn insert(&self, name: &str, version: &str, address: &str, abi: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![name.to_owned(), version.to_owned(), address.to_owned(), abi.to_owned()];
        send_transaction(
            self.web3_service,
            "CNSPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "insert",
            &params
        ).await
    }

    pub async fn select_by_name(&self, name: &str) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![name.to_owned()];
        let response = call(
            self.web3_service,
            "CNSPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "selectByName",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn select_by_name_and_version(&self, name: &str, version: &str) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![name.to_owned(), version.to_owned()];
        let response = call(
            self.web3_service,
            "CNSPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "selectByNameAndVersion",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn get_contract_address(&self, name: &str, version: &str) -> Result<String, PrecompiledServiceError> {
        let params = vec![name.to_owned(), version.to_owned()];
        let response = call(
            self.web3_service,
            "CNSPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "getContractAddress",
            &params
        ).await?;
        parse_address_token_to_string(&response.output)
    }
}