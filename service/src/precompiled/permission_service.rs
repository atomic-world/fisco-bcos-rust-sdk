use serde_json::Value as JSONValue;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{
    PrecompiledServiceError,
    call,
    send_transaction,
    parse_string_token_to_json,
};

const ADDRESS: &str = "0x0000000000000000000000000000000000001005";
const ABI_CONTENT: &str = r#"[{"constant":false,"inputs":[{"name":"table_name","type":"string"},{"name":"addr","type":"string"}],"name":"insert","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"table_name","type":"string"}],"name":"queryByName","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"table_name","type":"string"},{"name":"addr","type":"string"}],"name":"remove","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"contractAddr","type":"address"}],"name":"queryPermission","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"contractAddr","type":"address"},{"name":"user","type":"address"}],"name":"grantWrite","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"contractAddr","type":"address"},{"name":"user","type":"address"}],"name":"revokeWrite","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

pub struct PermissionService<'l> {
    web3_service: &'l Web3Service,
}

impl<'l> PermissionService<'l> {
    pub fn new(web3_service: &'l Web3Service) -> PermissionService<'l> {
        PermissionService {
            web3_service
        }
    }

    pub async fn insert(&self, table_name: &str, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![table_name.to_owned(), user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "PermissionPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "insert",
            &params
        ).await
    }

    pub async fn remove(&self, table_name: &str, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![table_name.to_owned(), user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "PermissionPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "remove",
            &params
        ).await
    }

    pub async fn query_by_name(&self, table_name: &str) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![table_name.to_owned()];
        let response = call(
            self.web3_service,
            "PermissionPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "queryByName",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }

    pub async fn grant_write(&self, contract_address: &str, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![contract_address.to_owned(), user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "PermissionPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "grantWrite",
            &params
        ).await
    }

    pub async fn revoke_write(&self, contract_address: &str, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![contract_address.to_owned(), user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "PermissionPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "revokeWrite",
            &params
        ).await
    }

    pub async fn query_permission(&self, contract_address: &str) -> Result<JSONValue, PrecompiledServiceError> {
        let params = vec![contract_address.to_owned()];
        let response = call(
            self.web3_service,
            "PermissionPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "queryPermission",
            &params
        ).await?;
        parse_string_token_to_json(&response.output)
    }
}