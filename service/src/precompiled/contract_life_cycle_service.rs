use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{PrecompiledServiceError, call, send_transaction, parse_output};

const ADDRESS: &str = "0x0000000000000000000000000000000000001007";
const ABI_CONTENT: &str = r#"[{"constant":true,"inputs":[{"name":"addr","type":"address"}],"name":"getStatus","outputs":[{"name":"","type":"int256"},{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"addr","type":"address"}],"name":"unfreeze","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"addr","type":"address"}],"name":"freeze","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"contractAddr","type":"address"},{"name":"userAddr","type":"address"}],"name":"grantManager","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"addr","type":"address"}],"name":"listManager","outputs":[{"name":"","type":"int256"},{"name":"","type":"address[]"}],"payable":false,"stateMutability":"view","type":"function"}]"#;

pub struct ContractLifeCycleService<'a> {
    web3_service: &'a Web3Service,
}

impl ContractLifeCycleService<'_> {
    pub fn new(web3_service: &Web3Service) -> ContractLifeCycleService {
        ContractLifeCycleService {
            web3_service
        }
    }

    pub async fn freeze(&self, contract_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![contract_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ContractLifeCyclePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "freeze",
            &params
        ).await
    }

    pub async fn unfreeze(&self, contract_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![contract_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ContractLifeCyclePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "unfreeze",
            &params
        ).await
    }

    pub async fn grant_manager(&self, contract_address: &str, user_address: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![contract_address.to_owned(), user_address.to_owned()];
        send_transaction(
            self.web3_service,
            "ContractLifeCyclePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "grantManager",
            &params
        ).await
    }

    pub async fn get_status(&self, contract_address: &str) -> Result<(i32, String), PrecompiledServiceError> {
        let params = vec![contract_address.to_owned()];
        let response = call(
            self.web3_service,
            "ContractLifeCyclePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "getStatus",
            &params
        ).await?;
        let tokens = response.output.unwrap();
        let code = parse_output(&tokens[0].clone().into_int().unwrap())?;
        let message = tokens[1].clone().into_string().unwrap();
        Ok((code, message))
    }

    pub async fn list_manager(&self, contract_address: &str) -> Result<(i32, Vec<String>), PrecompiledServiceError> {
        let params = vec![contract_address.to_owned()];
        let response = call(
            self.web3_service,
            "ContractLifeCyclePrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "listManager",
            &params
        ).await?;
        let tokens = response.output.unwrap();
        let code = parse_output(&tokens[0].clone().into_int().unwrap())?;
        let addresses = tokens[1].clone().into_array().unwrap_or(vec![])
            .into_iter()
            .map(|token| token.into_address())
            .filter(|address| address.is_some())
            .map(|address| { format!("{:?}", address) })
            .collect();
        Ok((code, addresses))
    }
}