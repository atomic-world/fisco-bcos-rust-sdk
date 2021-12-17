# fisco-bcos-rust-sdk

![logo](https://gitee.com/FISCO-BCOS/FISCO-BCOS/raw/master/docs/images/FISCO_BCOS_Logo.svg)

[![GitHub license](https://img.shields.io/badge/%20license-Apache%202.0-green)](https://github.com/CatLibrary/fisco-bcos-rust-sdk/blob/main/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/CatLibrary/fisco-bcos-rust-sdk.svg)](https://github.com/CatLibrary/fisco-bcos-rust-sdk/issues)
[![Code Lines](https://tokei.rs/b1/github/CatLibrary/fisco-bcos-rust-sdk)](https://github.com/CatLibrary/fisco-bcos-rust-sdk)

____

Rust SDK 为联盟链平台 [FISCO BCOS](https://github.com/FISCO-BCOS/FISCO-BCOS) 提供面向 Rust 的应用程序接口，使用 Rust SDK 可以简单快捷地开发基于 FISCO-BCOS 的 Rust 应用。 Rust SDK **仅支持** 2.7.0 及以上版本的 [FISCO BCOS](https://github.com/FISCO-BCOS/FISCO-BCOS)。

# 关键特性

* 提供 [JSON-RPC](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/api.html) 接口。
* 提供[预编译合约](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/precompiled_contract.html)接口。
* 支持国密和非国密的 [Channel](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/design/protocol_description.html#channelmessage) 协议。
* 支持国密和非国密下部署、调用 Solidity 合约接口。
* 支持合约编译，可将 Solidity 合约编译成 abi 和 bin 文件。
* 提供交互式的 CLI 工具，支持用户部署及调用合约、管理区块链状态等操作。

# 使用

## 快速建链

```bash
# 获取建链脚本
curl -LO https://github.com/FISCO-BCOS/FISCO-BCOS/releases/download/`curl -s https://api.github.com/repos/FISCO-BCOS/FISCO-BCOS/releases | grep "\"v2\.[0-9]\.[0-9]\"" | sort -u | tail -n 1 | cut -d \" -f 4`/build_chain.sh && chmod u+x build_chain.sh
# 在本地建一个4节点的FISCO BCOS链
# 如果合约交易需启用国密，添加 -g 属性
# 如果 Channel 连接需启用国密，添加 -G 属性
bash build_chain.sh -l "127.0.0.1:4" -p 30300,20200,8545 -i
# 启动FISCO BCOS链
bash nodes/127.0.0.1/start_all.sh

# 将证书文件拷贝至证书配置目录下，此处假设您的证书配置目录为：~/workspace/fisco-bcos-rust-sdk/configs/authentication，请根据实际情况进行替换
cp nodes/127.0.0.1/sdk/* ~/workspace/fisco-bcos-rust-sdk/configs/authentication/
```
## CLI

详情参见：[fisco-bcos-cli](https://github.com/atomic-world/fisco-bcos-rust-sdk/tree/main/cli)
## API Service

详情参见：[fisco-bcos-service](https://github.com/atomic-world/fisco-bcos-rust-sdk/tree/main/service)

# TODO

* 支持 [AMOP](https://fisco-bcos-documentation.readthedocs.io/zh_CN/latest/docs/manual/amop_protocol.html)。
* 支持合约事件监听。

# License

[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)
