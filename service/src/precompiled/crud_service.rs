use std::collections::HashMap;
use serde_json::Value as JSONValue;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{
    PrecompiledServiceError,
    call,
    send_transaction,
    parse_string_token_to_json,
};

const ADDRESS: &str = "0x0000000000000000000000000000000000001002";
const ABI_CONTENT: &str = r#"[{"constant":false,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"entry","type":"string"},{"name":"condition","type":"string"},{"name":"","type":"string"}],"name":"update","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"tableName","type":"string"}],"name":"desc","outputs":[{"name":"","type":"string"},{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"condition","type":"string"},{"name":"","type":"string"}],"name":"select","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"entry","type":"string"},{"name":"","type":"string"}],"name":"insert","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"tableName","type":"string"},{"name":"key","type":"string"},{"name":"condition","type":"string"},{"name":"","type":"string"}],"name":"remove","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

pub enum ConditionOperator {
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
}

#[derive(Default)]
pub struct Condition {
    conditions: HashMap<String, HashMap<String, String>>,
}

impl Condition {
    pub fn get_conditions(&self) -> HashMap<String, HashMap<String, String>> {
        self.conditions.clone()
    }

    pub fn get_condition_keys(&self) -> Vec<String> {
        self.conditions.keys().map(|key| key.clone()).collect()
    }

    pub fn is_key_exist(&self, key: &str) -> bool {
        self.conditions.contains_key(key)
    }

    pub fn set_condition(&mut self, key: &str, value: &str, operator: ConditionOperator) {
        let condition = match operator {
            ConditionOperator::Eq => {
                let mut condition: HashMap<String, String> = HashMap::new();
                condition.insert("eq".to_owned(), value.to_owned());
                condition
            },
            ConditionOperator::NotEq => {
                let mut condition: HashMap<String, String> = HashMap::new();
                condition.insert("ne".to_owned(), value.to_owned());
                condition
            },
            ConditionOperator::Gt => {
                let mut condition: HashMap<String, String> = HashMap::new();
                condition.insert("gt".to_owned(), value.to_owned());
                condition
            },
            ConditionOperator::Lt => {
                let mut condition: HashMap<String, String> = HashMap::new();
                condition.insert("lt".to_owned(), value.to_owned());
                condition
            },
            ConditionOperator::GtEq => {
                let mut condition: HashMap<String, String> = HashMap::new();
                condition.insert("ge".to_owned(), value.to_owned());
                condition
            },
            ConditionOperator::LtEq => {
                let mut condition: HashMap<String, String> = HashMap::new();
                condition.insert("le".to_owned(), value.to_owned());
                condition
            },
        };
        self.conditions.remove(key);
        self.conditions.insert(key.to_owned(), condition);
    }

    pub fn remove_condition(&mut self, key: &str) {
        self.conditions.remove(key);
    }

    pub fn set_limit(&mut self, limit: u32, offset: u32) {
        let mut condition: HashMap<String, String> = HashMap::new();
        condition.insert("limit".to_owned(), format!("{:},{:}", limit, offset));
        self.conditions.remove("limit");
        self.conditions.insert("limit".to_owned(), condition);
    }

    pub fn remove_limit(&mut self) {
        self.remove_condition("limit");
    }
}

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

    pub async fn select(&self, table_name: &str, key_value: &str, condition: &Condition) -> Result<Vec<JSONValue>, PrecompiledServiceError> {
        let conditions = condition.get_conditions();
        let params = vec![
            table_name.to_owned(),
            key_value.to_owned(),
            serde_json::to_string(&conditions)?,
            String::from(""),
        ];
        let response = call(
            self.web3_service,
            "CRUDPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "select",
            &params
        ).await?;
        Ok(
            match parse_string_token_to_json(&response.output)?.as_array() {
                Some(list) => list.into_iter().map(|item| item.clone()).collect(),
                None => vec![],
            }
        )
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
        let key_field = tokens[0].clone()
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
        Ok((key_field, value_fields))
    }
}