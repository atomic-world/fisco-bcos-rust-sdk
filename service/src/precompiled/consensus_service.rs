use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{PrecompiledServiceError, send_transaction};

const ADDRESS: &str = "0x0000000000000000000000000000000000001003";
const ABI_CONTENT: &str = r#"[{"constant":false,"inputs":[{"name":"nodeID","type":"string"}],"name":"addObserver","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"nodeID","type":"string"}],"name":"remove","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"nodeID","type":"string"}],"name":"addSealer","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

pub struct ConsensusService<'l> {
    web3_service: &'l Web3Service,
}

impl<'l> ConsensusService<'l> {
    pub fn new(web3_service: &'l Web3Service) -> ConsensusService<'l> {
        ConsensusService {
            web3_service
        }
    }

    pub async fn add_sealer(&self, value: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![value.to_owned()];
        send_transaction(
            self.web3_service,
            "ConsensusPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "addSealer",
            &params
        ).await
    }

    pub async fn add_observer(&self, value: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![value.to_owned()];
        send_transaction(
            self.web3_service,
            "ConsensusPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "addObserver",
            &params
        ).await
    }

    pub async fn remove(&self, value: &str) -> Result<i32, PrecompiledServiceError> {
        let params = vec![value.to_owned()];
        send_transaction(
            self.web3_service,
            "ConsensusPrecompiled",
            ADDRESS,
            ABI_CONTENT,
            "remove",
            &params
        ).await
    }
}