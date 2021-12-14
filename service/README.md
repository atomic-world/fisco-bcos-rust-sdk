# fisco-bcos-service

Rust SDK For FISCO BCOS 2.7.0+

# 安装

```toml
[dependencies]
fisco-bcos-service = "0.2"
```

此 crate 使用了 [TASSL](https://github.com/jntass/TASSL) 来处理 `TLS` 连接，在 `Linux` 或 `Macos` 下无需做任何额外操作，其他环境下则需要指定以下环境变量：

* `TASSL_LIB_PATH`：lib 库加载路径。
* `TASSL_INCLUDE_PATH`：头文件检索路径。
* `TASSL_LIB_KIND`：lib 库类型，可用值为：`static` 或 `dylib`。

在 `Linux` 或 `Macos` 下，如果你已经编译好了 `TASSL`，也可以通过指定以上环境变量来缩短编译时间。

# 使用

  * [一、配置](#一配置)
  * [二、Web3Service](#二Web3Service)
     * [2.1 实例化](#21-实例化)
     * [2.2 接口](#22-接口)
  * [三、SystemConfigService](#三SystemConfigService)
     * [3.1 实例化](#31-实例化)
     * [3.2 接口](#32-接口)
  * [四、TableFactoryService](#四TableFactoryService)
     * [4.1 实例化](#41-实例化)
     * [4.2 接口](#42-接口)
  * [五、CRUDService](#五CRUDService)
     * [5.1 实例化](#51-实例化)
     * [5.2 接口](#52-接口)
  * [六、SQLService](#六SQLService)
     * [6.1 实例化](#61-实例化)
     * [6.2 接口](#62-接口)
  * [七、ConsensusService](#七ConsensusService)
     * [7.1 实例化](#71-实例化)
     * [7.2 接口](#72-接口)
  * [八、CNSService](#八CNSService)
     * [8.1 实例化](#81-实例化)
     * [8.2 接口](#82-接口)
  * [九、PermissionService](#九PermissionService)
     * [9.1 实例化](#91-实例化)
     * [9.2 接口](#92-接口)
  * [十、ContractLifeCycleService](#十ContractLifeCycleService)
     * [10.1 实例化](#101-实例化)
     * [10.2 接口](#102-接口)
  * [十一、ChainGovernanceService](#十一ChainGovernanceService)
     * [11.1 实例化](#111-实例化)
     * [11.2 接口](#112-接口)
  * [十二、KVTableService](#十二KVTableService)
     * [12.1 实例化](#121-实例化)
     * [12.2 接口](#122-接口)
  * [十三、注意事项](#十三注意事项)
## 一、配置

配置文件为包含以下信息的  `json` 文件：

```json
{
    "service_type": "rpc",
    "node": {
        "host": "127.0.0.1",
        "port": 8545
    },
    "account": "./accounts/alice.pem",
    "contract":  {
        "solc": "./bin/solc-0.4.25",
        "source": "./contracts",
        "output": "./contracts/.output"
    },
    "authentication": {
        "ca_cert": "./authentication/gm/gmca.crt",
        "sign_cert": "./authentication/gm/gmsdk.crt",
        "sign_key": "./authentication/gm/gmsdk.key",
        "enc_key": "./authentication/gm/gmensdk.key",
        "enc_cert": "./authentication/gm/gmensdk.crt"
    },
    "sm_crypto": false,
    "group_id": 1,
    "chain_id": 1,
    "timeout_seconds": 10
}
```

每一项的解释如下：

* `service_type`：服务类型，可用值为：`rpc` 或 `channel`。

* `node`：服务节点信息，包含以下属性：

    * `host`：服务节点主机地址。
    * `port`：服务节点端口号。

* `account`：用户证书文件路径（仅支持 `pem` 格式）。

* `contract`：合约相关配置，包含以下属性：

    * `solc`：Solidity 编译器所在路径。
    * `source`：Solidity 合约源文件所在路径。
    * `output`：Solidity 合约编译后的 `abi` 及 `bin` 文件输出目录（该目录需自行创建）。

* `authentication`：节点验证配置信息（仅在 `service_type` 为 `channel` 时设置），包含以下属性：

    * `ca_cert`：CA 证书文件路径。
    * `sign_cert`：签名证书文件路径。
    * `sign_key`：签名密钥文件路径。
    * `enc_key`：enc 密钥文件路径（`非国密`模式下无需设置）。
    * `enc_cert`：enc 证书文件路径（`非国密`模式下无需设置）。

* `sm_crypto`：交易签名是否使用`国密`。
* `group_id`：组 ID。
* `chain_id`：链 ID。
* `timeout_seconds`： 网络过期时间（单位为秒）。

**注：配置项中 `account`、`contract`、`authentication` 中的路径如果使用相对路径，它的参考路径为该配置文件所在路径。**


## 二、Web3Service

[Web3Service](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/web3/service.rs) 是对 [FISCO BCOS JSON-RPC](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html) 的封装。
### 2.1 实例化

```rust
use fisco_bcos_service::create_web3_service;

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
```

### 2.2 接口

* 接口列表：

    * `get_client_version`
    * `get_block_number`
    * `get_pbft_view`
    * `get_sealer_list`
    * `get_observer_list`
    * `get_consensus_status`
    * `get_sync_status`
    * `get_peers`
    * `get_group_peers`
    * `get_node_id_list`
    * `get_group_list`
    * `get_block_by_hash`
    * `get_block_by_number`
    * `get_block_header_by_hash`
    * `get_block_header_by_number`
    * `get_block_hash_by_number`
    * `get_transaction_by_hash`
    * `get_transaction_by_block_hash_and_index`
    * `get_transaction_by_block_number_and_index`
    * `get_transaction_receipt`
    * `get_pending_transactions`
    * `get_pending_tx_size`
    * `get_code`
    * `get_total_transaction_count`
    * `get_system_config_by_key`
    * `call`
    * `send_raw_transaction`
    * `send_raw_transaction_and_get_proof`
    * `deploy`
    * `compile`
    * `get_transaction_by_hash_with_proof`
    * `get_transaction_receipt_by_hash_with_proof`
    * `generate_group`
    * `start_group`
    * `stop_group`
    * `remove_group`
    * `recover_group`
    * `query_group_status`
    * `get_node_info`
    * `get_batch_receipts_by_block_number_and_range`
    * `get_batch_receipts_by_block_hash_and_range`

* 接口将 [FISCO BCOS JSON-RPC](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html) 返回的 `error` 属性转换成了 `fisco_bcos_service::web3::service_error::ServiceError::FiscoBcosError` 异常并返回，包含以下属性：

    * code：错误类型，详情参见：[错误码描述](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html#id2)。
    * message：错误信息。

* 除 `call` 的返回值结构与相关 JSON-RPC 方法不一致外，其余接口的返回值结构参见 [FISCO BCOS JSON-RPC](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html) 中相关方法返回值中的 `result` 属性。

* 调用 `call`、`send_raw_transaction`、`send_raw_transaction_and_get_proof`、`deploy` 之前，请确保相关合约的 `abi` 及 `bin` 文件已存放在配置属性 `contract.output` 中的指定目录下，你可点击以下链接 [download_solc.sh](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/bin/download_solc.sh) 下载编译器后自行编译，也可调用 `compile` 接口编译。

* `deploy` 的返回值结构如下所示：

  ```json
  {
    "status": "0x0",
    "transactionHash": "0x31ad4fd454fbe72557cbcb55bde067cfcd80fa43e9d97bdf2c13d2007f066370",
    "contractAddress": "0x62195d0f77f66c445c4878b845f55d266875705d"
  }
  ```

* 接口中的 `Token` 实为 `ethabi::token::Token`，具体使用参见 [ethabi token](https://github.com/rust-ethereum/ethabi/blob/v14.1.0/ethabi/src/token/token.rs#L227-L299)，在使用过程中无需安装 `ethabi` 依赖，只需引用 `fisco_bcos_service::ethabi::token::Token` 即可。


## 三、SystemConfigService

[SystemConfigService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/system_config_service.rs) 是对预编译合约 [SystemConfigPrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#systemconfigprecompiled-0x1000) 的封装。

### 3.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    precompiled::system_config_service::SystemConfigService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let system_config_service = SystemConfigService::new(&web3_service);
```

### 3.2 接口

* 接口列表：

    * `set_value_by_key`

* 以上接口的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。


## 四、TableFactoryService

[TableFactoryService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/table_factory_service.rs) 是对预编译合约 [TableFactory](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#tablefactoryprecompiled-0x1001) 的封装。

### 4.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    precompiled::table_factory_service::TableFactoryService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let table_factory_service = TableFactoryService::new(&web3_service);
```

### 4.2 接口

* 接口列表：

    * `open_table`
    * `create_table`

* 接口 `open_table` 的返回值为指定 `table` 所在地址，如果指定 `table` 不存在，返回：`0x0000000000000000000000000000000000000000`。
* 接口 `create_table` 的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。


## 五、CRUDService

[CRUDService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/crud_service.rs) 是对预编译合约 [CRUDPrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#crudprecompiled-0x1002) 的封装。

### 5.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    precompiled::crud_service::CRUDService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let crud_service = CRUDService::new(&web3_service);
```

### 5.2 接口

* 接口列表：

    * `insert`
    * `remove`
    * `select`
    * `update`
    * `desc`

* 接口 `insert`、`remove`、`update` 的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。

* 接口 `desc` 返回 `(String, Vec<String>)` 类型，其中第一个值为`主键字段`，第二个值为`普通字段`列表。

## 六、SQLService

通过 [SQLService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/sql_service.rs) ，我们可以以 `SQL` 语句的方式来操作上述 `TableFactoryService` 和 `CRUDService` 相关接口。

### 6.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    sql_service::SQLService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let sql_service = SQLService::new(&web3_service);
```

### 6.2 接口

* 接口列表：

    * `execute`

* 通过 `CREATE TABLE` 语句创建表单时，将忽略指定的字段类型，并且通过指定 `PRIMARY KEY`（有且仅有一个）来设置表单的 `key` 字段。
* 通过 `INSERT` 语句插入数据时，必须设置 `key` 字段的值。

## 七、ConsensusService

[ConsensusService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/consensus_service.rs) 是对预编译合约 [ConsensusPrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#consensusprecompiled-0x1003) 的封装。

### 7.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    precompiled::consensus_service::ConsensusService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let consensus_service = ConsensusService::new(&web3_service);
```

### 7.2 接口

* 接口列表：

    * `add_sealer`
    * `add_observer`
    * `remove`

* 以上接口的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。

## 八、CNSService

[CNSService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/cns_service.rs) 是对预编译合约 [CNSPrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#cnsprecompiled-0x1004) 的封装。

### 8.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    cns_service::CNSService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let cns_service = CNSService::new(&web3_service);
```

### 8.2 接口

* 接口列表：

    * `insert`
    * `select_by_name`
    * `select_by_name_and_version`
    * `get_contract_address`

* 接口 `insert` 的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。

* 接口 `select_by_name` 与 `select_by_name_and_version` 会将返回值由 `string` 转换成 `serde_json::Value` 格式。

* 接口 `get_contract_address` 会将返回值由 `address` 转换成 `String` 格式。

## 九、PermissionService

[PermissionService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/permission_service.rs) 是对预编译合约 [PermissionPrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#permissionprecompiled-0x1005) 的封装。

### 9.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    permission_service::PermissionService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let permission_service = PermissionService::new(&web3_service);
```

### 9.2 接口

* 接口列表：

    * `insert`
    * `remove`
    * `query_by_name`
    * `grant_write`
    * `revoke_write`
    * `query_permission`

* 接口 `insert`、`remove`、`grant_write`、 `revoke_write` 的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。

* 接口 `query_by_name` 与 `query_permission` 会将返回值由 `string` 转换成 `serde_json::Value` 格式。

## 十、ContractLifeCycleService

[ContractLifeCycleService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/contract_life_cycle_service.rs) 是对预编译合约 [ContractLifeCyclePrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#contractlifecycleprecompiled-0x1007) 的封装。

### 10.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    contract_life_cycle_service::ContractLifeCycleService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let contract_life_cycle_service = ContractLifeCycleService::new(&web3_service);
```

### 10.2 接口

* 接口列表：

    * `freeze`
    * `unfreeze`
    * `grant_manager`
    * `get_status`
    * `list_manager`

* 接口 `freeze`、`unfreeze`、`grant_manager` 的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。

* 接口 `get_status` 的返回值为 `(i32, String)`，其中如果 `i32` 的值小于 `0`，将返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常。

* 接口 `list_manager` 的返回值为 `(i32, Vec<String>)`，其中如果 `i32` 的值小于 `0`，将返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常。

## 十一、ChainGovernanceService

[ChainGovernanceService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/chain_governance_service.rs) 是对预编译合约 [ChainGovernancePrecompiled](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#chaingovernanceprecompiled-0x1008) 的封装。

### 11.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    chain_governance_service::ChainGovernanceService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let chain_governance_service = ChainGovernanceService::new(&web3_service);
```

### 11.2 接口

* 接口列表：

    * `grant_committee_member`
    * `revoke_committee_member`
    * `list_committee_members`
    * `query_committee_member_weight`
    * `update_committee_member_weight`
    * `query_votes_of_member`
    * `query_votes_of_threshold`
    * `update_threshold`
    * `query_threshold`
    * `grant_operator`
    * `revoke_operator`
    * `list_operators`
    * `freeze_account`
    * `unfreeze_account`
    * `get_account_status`


* 接口 `grant_committee_member`、`revoke_committee_member`、`update_committee_member_weight`、`update_threshold`、`query_threshold`、`grant_operator`、`revoke_operator`、`freeze_account`、`unfreeze_account` 的返回值如果大于等于 `0`，返回此值，否则返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常，包含以下属性：

    * code：错误类型。
    * message：错误信息。

* 接口 `query_committee_member_weight` 的返回值为 `(bool, i32)`，其中如果 `i32` 的值小于 `0`，将返回 `fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError` 异常。

* 接口 `list_committee_members`、`query_votes_of_member`、`query_votes_of_threshold`、`list_operators` 会将返回值由 `string` 转换成 `serde_json::Value` 格式。


## 十二、KVTableService

[KVTableService](https://github.com/atomic-world/fisco-bcos-rust-sdk/blob/main/service/src/precompiled/kv_table_service.rs) 是对预编译合约 [KVTable](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html#kvtablefactoryprecompiled-0x1010) 的封装。

### 12.1 实例化

```rust
use fisco_bcos_service::{
    create_web3_service,
    kv_table_service::KVTableService,
};

let config_file_path = "./configs/config.json";
let web3_service = create_web3_service(config_file_path).unwrap();
let kv_table_service = KVTableService::new(&web3_service);
```

### 12.2 接口

## 十三、注意事项

* 所有接口均为异步调用（使用了 Rust 的 [async](https://rust-lang.github.io/async-book/) 特性）。

# License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)
