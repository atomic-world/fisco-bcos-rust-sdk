use serde_json::{json, Value as JSONValue};

use crate::web3::service::{CallResponse, Service as Web3Service};
use crate::precompiled::precompiled_service::{PrecompiledServiceError, send_transaction, call};

const ADDRESS: &str = "0x0000000000000000000000000000000000001004";
const ABI_CONTENT: &str = r#"[{"constant":true,"inputs":[{"name":"name","type":"string"}],"name":"selectByName","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"name","type":"string"},{"name":"version","type":"string"}],"name":"selectByNameAndVersion","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"addr","type":"string"},{"name":"abi","type":"string"}],"name":"insert","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"name","type":"string"},{"name":"version","type":"string"}],"name":"getContractAddress","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"}]"#;

fn parse_call_response(response: &CallResponse) -> Result<JSONValue, PrecompiledServiceError> {
    Ok(
        match &response.output {
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

pub struct CNSService<'a> {
    web3_service: &'a Web3Service,
}

impl CNSService<'_> {
    pub fn new(web3_service: &Web3Service) -> CNSService {
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
        parse_call_response(&response)
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
        parse_call_response(&response)
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
        Ok(
            match &response.output {
                None => String::from(""),
                Some(tokens) => {
                    if tokens.len() > 0 {
                        match tokens[0].clone().into_address() {
                            Some(address) => format!("{:?}", address),
                            None => String::from(""),
                        }
                    } else {
                        String::from("")
                    }
                }
            }
        )
    }
}