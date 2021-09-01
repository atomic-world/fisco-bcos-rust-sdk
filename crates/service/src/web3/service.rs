use serde_json::{Value, json};
use crate::account::{Account, create_account_from_pem};
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
    account: Account,
    fetcher: Box<dyn FetcherTrait + Send + Sync>,
}

impl Service {
    pub fn new(group_id: u32, pem_file_path: &str, fetcher: Box<dyn FetcherTrait + Send + Sync>) -> Result<Service, ServiceError> {
        Ok(
            Service {
                group_id,
                fetcher,
                account: create_account_from_pem(pem_file_path)?,
            }
        )
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
        let params = generate_request_params("getNodeIDList", &json!([self.group_id]));
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

    pub async fn call(
        &self,
        abi_path: &str,
        to_address: &str,
        function_name: &str,
        params: &Vec<&str>,
    ) -> Result<Value, ServiceError> {
        Ok(json!("TO DO"))
    }

    pub async fn send_raw_transaction(
        &self,
        abi_path: &str,
        to_address: &str,
        function_name: &str,
        params: &Vec<&str>,
    ) -> Result<String, ServiceError> {
        Ok(json!("TO DO").to_string())
    }

    pub async fn send_raw_transaction_and_get_proof(
        &self,
        abi_path: &str,
        to_address: &str,
        function_name: &str,
        params: &Vec<&str>,
    ) -> Result<String, ServiceError> {
        Ok(json!("TO DO").to_string())
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

    ///
    /// params 中的属性如下所示：
    ///
    /// |  属性名 | 类型 | 备注 |
    /// |  ----  | ---- | ---- |
    /// | timestamp | u32 | 创世块时间戳 |
    /// | sealers   | Vec\<String\> | 共识节点列表，要求所有所列共识节点间存在有效的 P2P 连接 |
    /// | enable_free_storage | bool | 可选，是否启用 free storage 模式，启用后节点将减少 STORAGE 相关指令的 gas 耗费 |
    ///
    pub async fn generate_group(&self, params: &Value) -> Result<Value, ServiceError> {
        let request_params = json!([self.group_id, params.clone()]);
        Ok(self.fetcher.fetch(&generate_request_params("generateGroup", &request_params)).await?)
    }

    pub async fn start_group(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "startGroup",
            &json!([self.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn stop_group(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "stopGroup",
            &json!([self.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn remove_group(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "removeGroup",
            &json!([self.group_id]),
        );
        Ok(self.fetcher.fetch(&params).await?)
    }

    pub async fn recover_group(&self) -> Result<Value, ServiceError> {
        let params = generate_request_params(
            "recoverGroup",
            &json!([self.group_id]),
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
        let params = generate_request_params("getNodeInfo", &json!(null));
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