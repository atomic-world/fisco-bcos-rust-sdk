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

pub struct Service {
    group_id: u32,
    fetcher: Box<dyn FetcherTrait + Send + Sync>,
}

impl Service {
    pub fn new(group_id: u32, fetcher: Box<dyn FetcherTrait + Send + Sync>) -> Service {
        Service { group_id, fetcher}
    }

    pub async fn get_client_version(&self)  -> Result<Value, ServiceError> {
        let params = generate_request_params("getClientVersion", &json!([self.group_id]));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_block_number(&self) -> Result<String, ServiceError> {
        let params = generate_request_params("getBlockNumber", &json!([self.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_pbft_view(&self) -> Result<String, ServiceError> {
        let params = generate_request_params("getPbftView", &json!([self.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_sealer_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getSealerList", &json!([self.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_observer_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getObserverList", &json!([self.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_consensus_status(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params("getConsensusStatus", &json!([self.group_id]));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_sync_status(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params("getSyncStatus", &json!([self.group_id]));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_peers(&self) -> Result<Vec<Value>, ServiceError> {
        let params = generate_request_params("getPeers", &json!([self.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(response.as_array().unwrap().clone())
    }

    pub async fn get_group_peers(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getGroupPeers", &json!([self.group_id]));
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_array_value(&response))
    }

    pub async fn get_node_id_list(&self) -> Result<Vec<String>, ServiceError> {
        let params = generate_request_params("getGroupPeers", &json!([self.group_id]));
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

    pub async fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTransactionByHash",
            &json!([self.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, transaction_index: &str) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTransactionByBlockHashAndIndex",
            &json!([self.group_id, block_hash, transaction_index]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_by_block_number_and_index(&self, block_number: &str, transaction_index: &str) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTransactionByBlockNumberAndIndex",
            &json!([self.group_id, block_number, transaction_index]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTransactionReceipt",
            &json!([self.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_pending_transactions(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getPendingTransactions",
            &json!([self.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_pending_tx_size(&self) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getPendingTxSize",
            &json!([self.group_id]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_code(&self, address: &str) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getCode",
            &json!([self.group_id, address]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_total_transaction_count(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTotalTransactionCount",
            &json!([self.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_system_config_by_key(&self, key: &str) -> Result<String, ServiceError> {
        let params = generate_request_params(
            "getSystemConfigByKey",
            &json!([self.group_id, key]),
        );
        let response = self.fetcher.fetch(&params).await?;
        Ok(parse_serde_json_string_value(&response))
    }

    pub async fn get_transaction_by_hash_with_proof(&self, transaction_hash: &str) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTransactionByHashWithProof",
            &json!([self.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_transaction_receipt_by_hash_with_proof(&self, transaction_hash: &str) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getTransactionReceiptByHashWithProof",
            &json!([self.group_id, transaction_hash]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn query_group_status(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "queryGroupStatus",
            &json!([self.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_node_info(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params("getGroupList", &json!(null));
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_batch_receipts_by_block_number_and_range(
        &self,
        block_number: &str,
        from: u32,
        count: i32,
        compress_flag: bool,
    ) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getBatchReceiptsByBlockNumberAndRange",
            &json!([self.group_id, block_number, from, count, compress_flag]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn get_batch_receipts_by_block_hash_and_range(
        &self,
        block_hash: &str,
        from: u32,
        count: i32,
        compress_flag: bool,
    ) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "getBatchReceiptsByBlockHashAndRange",
            &json!([self.group_id, block_hash, from, count, compress_flag]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }
}