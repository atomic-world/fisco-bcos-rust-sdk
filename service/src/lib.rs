pub mod web3;
pub mod account;
pub mod abi;
pub mod transaction;
pub mod helpers;
pub mod tassl;
pub mod config;
pub mod precompiled;
pub mod event;
pub mod channel;
pub use ethabi;
pub use serde_json;

pub use config::create_config_with_file;
pub use web3::service::create_service_with_config as create_web3_service_with_config;

use web3::service::{
    Service as Web3Service,
    ServiceError as Web3ServiceError,
};

use event::event_service::EventService;

///
/// 根据配置文件创建 web3 service 服务实例。
///
/// 完整配置文件格式如下所示：
///
/// ```json
/// {
///    "service_type": "rpc",
///    "node": {
///        "host": "127.0.0.1",
///        "port": 8545
///    },
///    "account": "./accounts/alice.pem",
///    "contract":  {
///        "solc": "./bin/solc-0.4.25",
///        "source": "./contracts",
///        "output": "./contracts/.output"
///    },
///    "authentication": {
///        "ca_cert": "./authentication/gm/gmca.crt",
///        "sign_cert": "./authentication/gm/gmsdk.crt",
///        "sign_key": "./authentication/gm/gmsdk.key",
///        "enc_key": "./authentication/gm/gmensdk.key",
///        "enc_cert": "./authentication/gm/gmensdk.crt"
///    },
///    "sm_crypto": false,
///    "group_id": 1,
///    "chain_id": 1,
///    "timeout_seconds": 10
/// }
/// ```
///
pub fn create_web3_service(config_file_path: &str) -> Result<Web3Service, Web3ServiceError>  {
    let config = create_config_with_file(config_file_path)?;
    create_web3_service_with_config(&config)
}

///
/// 根据配置文件创建 event service 服务实例。
///
/// 完整配置文件格式如下所示：
///
/// ```json
/// {
///    "service_type": "rpc",
///    "node": {
///        "host": "127.0.0.1",
///        "port": 8545
///    },
///    "account": "./accounts/alice.pem",
///    "contract":  {
///        "solc": "./bin/solc-0.4.25",
///        "source": "./contracts",
///        "output": "./contracts/.output"
///    },
///    "authentication": {
///        "ca_cert": "./authentication/gm/gmca.crt",
///        "sign_cert": "./authentication/gm/gmsdk.crt",
///        "sign_key": "./authentication/gm/gmsdk.key",
///        "enc_key": "./authentication/gm/gmensdk.key",
///        "enc_cert": "./authentication/gm/gmensdk.crt"
///    },
///    "sm_crypto": false,
///    "group_id": 1,
///    "chain_id": 1,
///    "timeout_seconds": 10
/// }
/// ```
///
pub fn create_event_service(config_file_path: &str) -> Result<EventService, std::io::Error> {
    let config = create_config_with_file(&config_file_path)?;
    Ok(EventService::new(&config))
}