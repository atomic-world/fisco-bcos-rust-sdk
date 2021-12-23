use async_trait::async_trait;
use serde_json::Value as JSONValue;
use crate::web3::service::ServiceError;

pub fn parse_response(response: &JSONValue) -> Result<JSONValue, ServiceError> {
    let result = &response["result"];
    let error = &response["error"];
    match error.is_null() {
        true => Ok(result.clone()),
        false => {
            Err(ServiceError::FiscoBcosError {
                code: error["code"].as_i64().unwrap() as i32,
                message: error["message"].to_string(),
            })
        }
    }
}

#[async_trait]
pub trait FetcherTrait {
    async fn fetch(&self, params: &JSONValue) -> Result<JSONValue, ServiceError>;
}