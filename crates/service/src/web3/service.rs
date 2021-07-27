use serde_json::{Value, json};
use crate::web3::{fetcher_trait::FetcherTrait, service_error::ServiceError};
use crate::helpers::{parse_serde_json_string_value, parse_serde_json_string_array_value};

fn generate_request_params(method: &str, params: &Value) -> Value {
    json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": method.to_owned(),
        "params": params.clone(),
    })
}

fn generate_request_params_by_group_id(method: &str, group_id: u32) -> Value {
    let params = json!([group_id]);
    generate_request_params(method, &params)
}

pub struct Service {
    group_id: u32,
    fetcher: Box<dyn FetcherTrait + Send + Sync>,
}

impl Service {
    pub fn new(group_id: u32, fetcher: Box<dyn FetcherTrait + Send + Sync>) -> Service {
        Service { group_id, fetcher}
    }

    pub async fn get_client_version(&self)  -> Result<Value, ServiceError> {
        let params = generate_request_params_by_group_id("getClientVersion", self.group_id);
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_number(&self) -> Result<String, ServiceError> {
        let params = generate_request_params_by_group_id("getBlockNumber", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_pbft_view(&self) -> Result<String, ServiceError> {
        let params = generate_request_params_by_group_id("getPbftView", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_sealer_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getSealerList", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_observer_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getObserverList", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_consensus_status(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params_by_group_id("getConsensusStatus", self.group_id);
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_sync_status(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params_by_group_id("getSyncStatus", self.group_id);
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_peers(&self) -> Result<Vec<Value>, ServiceError> {
        let params = generate_request_params_by_group_id("getPeers", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(response.as_array().unwrap().clone())
    }

    pub async fn get_group_peers(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getGroupPeers", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_node_id_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params_by_group_id("getNodeIDList", self.group_id);
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_group_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getGroupList", &json!(null));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_block_by_hash(&self, block_hash: &str, include_transactions: bool) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getBlockByHash",
            &json!([self.group_id, block_hash, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_by_number(&self, block_number: &str, include_transactions: bool)-> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getBlockByNumber",
            &json!([self.group_id, block_number, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_header_by_hash(&self, block_hash: &str, include_transactions: bool) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getBlockHeaderByHash",
            &json!([self.group_id, block_hash, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_header_by_number(&self, block_number: &str, include_transactions: bool) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getBlockHeaderByNumber",
            &json!([self.group_id, block_number, include_transactions]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_hash_by_number(&self, block_number: &str) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getBlockHashByNumber",
            &json!([self.group_id, block_number]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }
}