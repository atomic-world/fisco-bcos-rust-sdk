use num_bigint::BigInt;
use async_trait::async_trait;
use serde_json::{Value, json};

use crate::service_error::ServiceError;

fn generate_request_params(method: &str, params: &Value) -> Value {
    json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": method.to_owned(),
        "params": params.clone(),
    })
}

fn generate_request_params_by_group_id(method: &str, group_id: u32) -> Value {
    let group_ids = [group_id];
    let params = json!(group_ids);
    generate_request_params(method, &params)
}

fn convert_response_to_big_int(response: &Value) -> BigInt {
    let bytes = response.as_str().unwrap().trim_start_matches("0x").as_bytes();
    BigInt::parse_bytes(bytes, 16).unwrap()
}

fn convert_response_to_string_array(response: &Value) -> Vec<String> {
    response.as_array().unwrap().into_iter().map(|item| item.to_string()).collect()
}

#[async_trait]
pub trait ServiceTrait {
    async fn fetch(&self, params: &Value) -> Result<Value, ServiceError>;

    async fn get_client_version(&self, group_id: u32) -> Result<Value, ServiceError> {
        let params = generate_request_params_by_group_id("getClientVersion", group_id);
        Ok(self.fetch(&params).await?)
    }

    async fn get_pbft_view(&self, group_id: u32) -> Result<BigInt, ServiceError> {
        let params = generate_request_params_by_group_id("getPbftView", group_id);
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_big_int(&response))
    }

    async fn get_block_number(&self, group_id: u32) -> Result<BigInt, ServiceError> {
        let params = generate_request_params_by_group_id("getBlockNumber", group_id);
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_big_int(&response))
    }

    async fn get_sealer_list(&self, group_id: u32) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getSealerList", group_id);
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_string_array(&response))
    }

    async fn get_observer_list(&self, group_id: u32) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getObserverList", group_id);
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_string_array(&response))
    }

    async fn get_consensus_status(&self, group_id: u32) -> Result<Value, ServiceError> {
        let params = generate_request_params_by_group_id("getConsensusStatus", group_id);
        Ok(self.fetch(&params).await?)
    }

    async fn get_sync_status(&self, group_id: u32) -> Result<Value, ServiceError> {
        let params = generate_request_params_by_group_id("getSyncStatus", group_id);
        Ok(self.fetch(&params).await?)
    }

    async fn get_peers(&self, group_id: u32) -> Result<Vec<Value>, ServiceError> {
        let params = generate_request_params_by_group_id("getPeers", group_id);
        let response = self.fetch(&params).await?;
        Ok(response.as_array().unwrap().clone())
    }

    async fn get_group_peers(&self, group_id: u32) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getGroupPeers", group_id);
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_string_array(&response))
    }

    async fn get_node_id_list(&self, group_id: u32) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getNodeIDList", group_id);
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_string_array(&response))
    }

    async fn get_group_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getGroupList", &json!(null));
        let response = self.fetch(&params).await?;
        Ok(convert_response_to_string_array(&response))
    }
}