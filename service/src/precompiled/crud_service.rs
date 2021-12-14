use std::collections::HashMap;
use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{
    PrecompiledServiceError,
    call,
    send_transaction,
};

const ADDRESS: &str = "0x0000000000000000000000000000000000001002";
const ABI_CONTENT: &str = r#"[{"constant":false,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"entry","type":"string"},{"name":"condition","type":"string"},{"name":"","type":"string"}],"name":"update","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"tableName","type":"string"}],"name":"desc","outputs":[{"name":"","type":"string"},{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"condition","type":"string"},{"name":"","type":"string"}],"name":"select","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"entry","type":"string"},{"name":"","type":"string"}],"name":"insert","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"condition","type":"string"},{"name":"","type":"string"}],"name":"remove","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

pub struct CRUDService<'a> {
    web3_service: &'a Web3Service,
}

impl CRUDService<'_> {
    pub fn new(web3_service: &Web3Service) -> CRUDService {
        CRUDService {
            web3_service
        }
    }

    pub async fn insert(&self, table_name: &str, key_value: &str, entry: &HashMap<String, String>) -> Result<i32, PrecompiledServiceError> {
        let params = vec![
            table_name.to_owned(),
            key_value.to_owned(),
            serde_json::to_string(&entry)?,
            String::from(""),
        ];
        send_transaction(
            self.web3_service,
            "CRUDPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "insert",
            &params
        ).await
    }

    pub async fn desc(&self, table_name: &str) -> Result<(String, Vec<String>), PrecompiledServiceError> {
        let params = vec![table_name.to_owned()];
        let response = call(
            self.web3_service,
            "CRUDPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "desc",
            &params
        ).await?;
        let tokens = response.output.unwrap();
        let primary_key = tokens[0].clone()
            .into_string()
            .unwrap_or(String::from(""));
        let value_fields: Vec<String> = tokens[1].clone()
            .into_string()
            .unwrap_or(String::from(""))
            .split(",")
            .into_iter()
            .map(|v| v.to_string())
            .filter(|v| v.ne(""))
            .collect();
        Ok((primary_key, value_fields))
    }
}