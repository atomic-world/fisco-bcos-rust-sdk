# fisco-bcos-cli

Rust 版 FISCO BCOS 可交互式命令行工具。

# 安装

```shell
$ cargo install --force --version '>=0.4, <1' fisco-bcos-cli
```

此 crate 使用了 [TASSL](https://github.com/jntass/TASSL) 来处理 `TLS` 连接，在 `Linux` 或 `Macos` 下无需做任何额外操作，其他环境下则需要指定以下环境变量：

* `TASSL_LIB_PATH`：lib 库加载路径。
* `TASSL_INCLUDE_PATH`：头文件检索路径。
* `TASSL_LIB_KIND`：lib 库类型，可用值为：`static` 或 `dylib`。

在 `Linux` 或 `Macos` 下，如果你已经编译好了 `TASSL`，也可以通过指定以上环境变量来缩短编译时间。

# 使用

```shell
$ fisco-bcos-cli
```

执行上述命令，你将进入以下交互页面：

```shell
=============================================================================================
Welcome to FISCO BCOS console(0.4.0).
Type 'help' for help.
Type 'CTRL-C' or 'CTRL-D' to quit console.
Visit https://github.com/kkawakam/rustyline#actions to get more actions.

________ ______  ______   ______   ______       _______   ______   ______   ______
|        |      \/      \ /      \ /      \     |       \ /      \ /      \ /      \
| $$$$$$$$\$$$$$|  $$$$$$|  $$$$$$|  $$$$$$\    | $$$$$$$|  $$$$$$|  $$$$$$|  $$$$$$\
| $$__     | $$ | $$___\$| $$   \$| $$  | $$    | $$__/ $| $$   \$| $$  | $| $$___\$$
| $$  \    | $$  \$$    \| $$     | $$  | $$    | $$    $| $$     | $$  | $$\$$    \
| $$$$$    | $$  _\$$$$$$| $$   __| $$  | $$    | $$$$$$$| $$   __| $$  | $$_\$$$$$$\
| $$      _| $$_|  \__| $| $$__/  | $$__/ $$    | $$__/ $| $$__/  | $$__/ $|  \__| $$
| $$     |   $$ \\$$    $$\$$    $$\$$    $$    | $$    $$\$$    $$\$$    $$\$$    $$
 \$$      \$$$$$$ \$$$$$$  \$$$$$$  \$$$$$$      \$$$$$$$  \$$$$$$  \$$$$$$  \$$$$$$

=============================================================================================

>>
```

输入 `help`，获取帮助信息：

```shell
>> help

1. Use set_config to initialize environment(e.g., set_config ./config/config.json).
2. Use the below APIs to interact with FISCO BCOS：

* get_client_version                                         Query the current node version.
* get_block_number                                           Query the number of most recent block.
* get_pbft_view                                              Query the pbft view of node.
* get_sealer_list                                            Query nodeId list for sealer nodes.
* get_observer_list                                          Query nodeId list for observer nodes.
* get_consensus_status                                       Query consensus status.
* get_sync_status                                            Query sync status.
* get_peers                                                  Query peers currently connected to the client.
* get_group_peers                                            Query nodeId list for sealer and observer nodes.
* get_node_id_list                                           Query nodeId list for all connected nodes.
* get_group_list                                             Query group list.
* get_block_by_hash                                          Query information about a block by hash.
* get_block_by_number                                        Query information about a block by number.
* get_block_header_by_hash                                   Query information about a block header by hash.
* get_block_header_by_number                                 Query information about a block header by block number.
* get_block_hash_by_number                                   Query block hash by block number.
* get_transaction_by_hash                                    Query information about a transaction requested by transaction hash.
* get_transaction_by_block_hash_and_index                    Query information about a transaction by block hash and transaction index position.
* get_transaction_by_block_number_and_index                  Query information about a transaction by block number and transaction index position.
* get_transaction_receipt                                    Query the receipt of a transaction by transaction hash.
* get_pending_transactions                                   Query pending transactions.
* get_pending_tx_size                                        Query pending transactions size.
* get_code                                                   Query code at a given address.
* get_total_transaction_count                                Query total transaction count.
* get_system_config_by_key                                   Query a system config value by key.
* call                                                       Call a contract by a function and parameters.
* send_raw_transaction                                       Execute a signed transaction with a contract function and parameters.
* send_raw_transaction_and_get_proof                         Execute a signed transaction with a contract function and parameters.
* deploy                                                     Deploy a contract on blockchain.
* compile                                                    Compile sol file to abi & bin files.
* get_transaction_by_hash_with_proof                         Query the transaction and transaction proof by transaction hash.
* get_transaction_receipt_by_hash_with_proof                 Query the receipt and transaction receipt proof by transaction hash.
* generate_group                                             Generate a group for the specified node.
* start_group                                                Start the specified group of the specified node.
* stop_group                                                 Stop the specified group of the specified node.
* remove_group                                               Remove the specified group of the specified node.
* recover_group                                              Recover the specified group of the specified node.
* query_group_status                                         Query the status of the specified group of the specified node.
* get_node_info                                              Query the specified node information.
* get_batch_receipts_by_block_number_and_range               Get batched transaction receipts according to block number and the transaction range.
* get_batch_receipts_by_block_hash_and_range                 Get batched transaction receipts according to block hash and the transaction range.
* system_config:set_value_by_key                             SystemConfigPrecompiled: Set a system config value by key.
* consensus:add_sealer                                       ConsensusPrecompiled: Add a sealer node.
* consensus:add_observer                                     ConsensusPrecompiled: Add an observer node.
* consensus:remove                                           ConsensusPrecompiled: Remove a node.
* cns:insert                                                 CNSPrecompiled: Insert CNS information for the given contract
* cns:select_by_name                                         CNSPrecompiled: Query CNS information by contract name.
* cns:select_by_name_and_version                             CNSPrecompiled: Query CNS information by contract name and contract version.
* cns:get_contract_address                                   CNSPrecompiled: Query contract address by contract name.
* permission:insert                                          PermissionPrecompiled: Grant the specified account write permission for the specified table.
* permission:remove                                          PermissionPrecompiled: Remove the specified account write permission for the specified table.
* permission:query_by_name                                   PermissionPrecompiled: Query the accounts who have write permission for the specified table.
* permission:grant_write                                     PermissionPrecompiled: Grant the specified account write permission for the specified contract.
* permission:revoke_write                                    PermissionPrecompiled: Revoke the specified account write permission for the specified contract.
* permission:query_permission                                PermissionPrecompiled: Query the accounts who have write permission for the specified contract.
* contract_life_cycle:freeze                                 ContractLifeCyclePrecompiled: Freeze the specified contract.
* contract_life_cycle:unfreeze                               ContractLifeCyclePrecompiled: Unfreeze the specified contract.
* contract_life_cycle:grant_manager                          ContractLifeCyclePrecompiled: Authorize a account to be the manager of the contract.
* contract_life_cycle:get_status                             ContractLifeCyclePrecompiled: Query the status of the specified contract.
* contract_life_cycle:list_manager                           ContractLifeCyclePrecompiled: Query the managers of the specified contract.
* chain_governance_service:grant_committee_member            ChainGovernancePrecompiled: Grant the account committee member.
* chain_governance_service:revoke_committee_member           ChainGovernancePrecompiled: Revoke the account from committee member.
* chain_governance_service:list_committee_members            ChainGovernancePrecompiled: List all committee members.
* chain_governance_service:query_committee_member_weight     ChainGovernancePrecompiled: Query the committee member weight.
* chain_governance_service:update_committee_member_weight    ChainGovernancePrecompiled: Update the committee member weight.
* chain_governance_service:query_votes_of_member             ChainGovernancePrecompiled: Query votes of a committee member.
* chain_governance_service:query_votes_of_threshold          ChainGovernancePrecompiled: Query votes of updateThreshold operation.
* chain_governance_service:update_threshold                  ChainGovernancePrecompiled: Update the threshold.
* chain_governance_service:query_threshold                   ChainGovernancePrecompiled: Query the threshold.
* chain_governance_service:grant_operator                    ChainGovernancePrecompiled: Grant the operator.
* chain_governance_service:revoke_operator                   ChainGovernancePrecompiled: Revoke the operator.
* chain_governance_service:list_operators                    ChainGovernancePrecompiled: List all operators.
* chain_governance_service:freeze_account                    ChainGovernancePrecompiled: Freeze the contract
* chain_governance_service:unfreeze_account                  ChainGovernancePrecompiled: Unfreeze the contract.
* chain_governance_service:get_account_status                ChainGovernancePrecompiled: Get the contract status.
* sql                                                        Execute CRUD operations with SQL.
```

首先调用 `set_config` 来设置环境信息（配置信息详情参见：[服务配置](https://github.com/atomic-world/fisco-bcos-rust-sdk/tree/fisco-2.x/service#%E4%B8%80%E9%85%8D%E7%BD%AE)），比如：

```shell
>> set_config ./configs/config.json
```

而后便可调用帮助信息中列出的方法对链上数据进行交互，比如：

``` shell
>> get_client_version

Object({"Build Time": String("20210201 10:15:37"), "Build Type": String("Darwin/appleclang/RelWithDebInfo"), "Chain Id": String("1"), "FISCO-BCOS Version": String("2.7.2"), "Git Branch": String("HEAD"), "Git Commit Hash": String("4c8a5bbe44c19db8a002017ff9dbb16d3d28e9da"), "Supported Version": String("2.7.2")})
```

交互方法的参数信息参见：[FISCO BCOS Service](https://github.com/atomic-world/fisco-bcos-rust-sdk/tree/fisco-2.x/service#%E4%BD%BF%E7%94%A8)。

注意事项：

* 方法名与参数、参数与参数之间以`空格`分割（比如：`call HelloWorldV4 0x62195d0f77f66c445c4878b845f55d266875705d get`），如果某个参数内部有空格或为其它更复杂的格式（比如 `JSON` 字符串），使用`单引号`包裹（比如：`send_raw_transaction HelloWorldV4 0x62195d0f77f66c445c4878b845f55d266875705d set 'hello world'`）。
* `call`、`send_raw_transaction`、`send_raw_transaction_and_get_proof`、`deploy` 方法的签名最后一个参数为 `Vec<Token>`，在调用时直接将其拆分为多个参数，然后以空格分开即可（比如：`send_raw_transaction Person 0x62195d0f77f66c445c4878b845f55d266875705d set 12 Tom`）。
* `compile` 方法签名的最后一个参数（需要链接的 `libraries`，该参数可不设置）为 `HashMap<String, String>`，在调用时请以 `JSON` 字符串的形式传递（比如：`compile HelloWorldV4 '{"MyLibrary": "0x123456..."}'`）。
* `generate_group` 的参数类型为 `serde_json::Value`，在调用时请以 `JSON` 字符串的形式传递。

# License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)
