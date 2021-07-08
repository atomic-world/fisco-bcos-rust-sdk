use async_trait::async_trait;
use serde_json::{Value, json};

use crate::service_error::ServiceError;

#[async_trait]
pub trait ServiceTrait {
    async fn fetch(&self, params: &Value) -> Result<Value, ServiceError>;

    async fn start_request(&self, method: &str, params: &Value) -> Result<Value, ServiceError> {
        let request_data = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": method.to_owned(),
            "params": params.to_owned(),
        });
        self.fetch(&request_data).await
    }

    async fn get_block_number(&self, group_id: u32) -> Result<u64, ServiceError> {
        let group_ids= [group_id];
        let params = json!(group_ids);
        let response = self.start_request("getBlockNumber", &params).await?;
        let block_number_hex = response["result"].as_str().unwrap().trim_start_matches("0x");
        let block_number = u64::from_str_radix(block_number_hex, 16)?;
        Ok(block_number)
    }
}