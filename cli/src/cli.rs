use std::{
    str::FromStr,
    collections::HashMap,
};
use fisco_bcos_service::{
    create_web3_service,
    abi::ABI,
    config::Config,
    ethabi::token::Token,
    serde_json::{json, Value as JSONValue},
    precompiled::{
        cns_service::CNSService,
        sql_service::SQLService,
        consensus_service::ConsensusService,
        permission_service::PermissionService,
        system_config_service::SystemConfigService,
        chain_governance_service::ChainGovernanceService,
        contract_life_cycle_service::ContractLifeCycleService,
    },
    web3::{service::Service as Web3Service, service_error::ServiceError as Web3ServiceError},
};
use fisco_bcos_service::precompiled::precompiled_service::PrecompiledServiceError;

fn valid_args_len(args_length: usize, min_len: usize) -> Result<(), Web3ServiceError> {
    if args_length < min_len {
        Err(Web3ServiceError::CustomError {
            message: format!("Argument count should not less than {:}", min_len),
        })
    } else {
        Ok(())
    }
}

fn convert_str_to_bool(value: &str) -> bool {
    value.eq_ignore_ascii_case("true")
}

fn convert_str_to_number<T:FromStr>(value: &str, default: T) -> T {
    String::from(value).parse::<T>().unwrap_or(default)
}

fn convert_str_to_json(value: &str) -> JSONValue {
    fisco_bcos_service::serde_json::from_str::<JSONValue>(value).unwrap_or(json!(null))
}

fn parse_contract_function_tokens(args: &Vec<String>, config: &Option<Config>) -> Vec<Token> {
    match config {
        None => vec![],
        Some(config) => {
            let args_length = args.len();
            let params: Vec<String> = if args_length > 3 {
                Vec::from(&args[3..])
            } else {
                vec![]
            };
            match ABI::new_with_contract_config(&config.contract, &args[0], config.sm_crypto) {
                Ok(abi) =>  abi.parse_function_tokens(&args[2], &params).unwrap_or(vec![]),
                Err(error) => {
                    println!("\nError: {:?}\n", error);
                    vec![]
                },
            }
        }
    }
}

fn parse_contract_constructor_tokens(args: &Vec<String>, config: &Option<Config>) -> Vec<Token> {
    match config {
        None => vec![],
        Some(config) => {
            let args_length = args.len();
            let params: Vec<String> = if args_length > 2 {
                Vec::from(&args[2..])
            } else {
                vec![]
            };
            match ABI::new_with_contract_config(&config.contract, &args[0], config.sm_crypto) {
                Ok(abi) =>   abi.parse_constructor_tokens(&params).unwrap_or(vec![]),
                Err(error) => {
                    println!("\nError: {:?}\n", error);
                    vec![]
                },
            }
        }
    }
}

pub(crate) struct Cli {
    config: Option<Config>,
    web3_service: Option<Web3Service>,
}

impl Cli {
    fn set_config(&mut self, config_path: &str) {
        match create_web3_service(config_path) {
            Ok(web3_service) => {
                self.config = Some(web3_service.get_config());
                self.web3_service = Some(web3_service);
            },
            Err(error) => println!("\n Web3 Service initialize error: {:?}\n", error),
        };
    }

    async fn call_web3_service(&self, method: &str, args: &Vec<String>) {
        let args_length = args.len();
        let web3_service = self.web3_service.as_ref().unwrap();
        let response = match method {
            "get_client_version" => web3_service.get_client_version().await,
            "get_block_number" => web3_service.get_block_number().await.map(|v| json!(v)),
            "get_pbft_view" => web3_service.get_pbft_view().await.map(|v| json!(v)),
            "get_sealer_list" => web3_service.get_sealer_list().await.map(|v| json!(v)),
            "get_observer_list" => web3_service.get_observer_list().await.map(|v| json!(v)),
            "get_consensus_status" => web3_service.get_consensus_status().await,
            "get_sync_status" => web3_service.get_sync_status().await,
            "get_peers" => web3_service.get_peers().await.map(|v| json!(v)),
            "get_group_peers" => web3_service.get_group_peers().await.map(|v| json!(v)),
            "get_node_id_list" => web3_service.get_node_id_list().await.map(|v| json!(v)),
            "get_group_list" => web3_service.get_group_list().await.map(|v| json!(v)),
            "get_block_by_hash" => {
               match valid_args_len(args_length, 2) {
                   Err(err) => Err(err),
                   Ok(_) => web3_service.get_block_by_hash(
                       &args[0],
                       convert_str_to_bool(&args[1])
                   ).await,
               }
            },
            "get_block_by_number" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_block_by_number(
                        &args[0],
                        convert_str_to_bool(&args[1])
                    ).await,
                }
            },
            "get_block_header_by_hash" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_block_header_by_hash(
                        &args[0],
                        convert_str_to_bool(&args[1])
                    ).await,
                }
            },
            "get_block_header_by_number" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_block_header_by_number(
                        &args[0],
                        convert_str_to_bool(&args[1])
                    ).await,
                }
            },
            "get_block_hash_by_number" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_block_hash_by_number(&args[0])
                        .await.map(|v| json!(v)),
                }
            },
            "get_transaction_by_hash" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_transaction_by_hash(&args[0]).await,
                }
            },
            "get_transaction_by_block_hash_and_index" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_transaction_by_block_hash_and_index(
                        &args[0],
                        &args[1],
                    ).await,
                }
            },
            "get_transaction_by_block_number_and_index" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_transaction_by_block_number_and_index(
                        &args[0],
                        &args[1],
                    ).await,
                }
            },
            "get_transaction_receipt" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_transaction_receipt(&args[0]).await,
                }
            },
            "get_pending_transactions" => web3_service.get_pending_transactions().await,
            "get_pending_tx_size" => web3_service.get_pending_tx_size().await.map(|v| json!(v)),
            "get_code" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_code(&args[0]).await.map(|v| json!(v)),
                }
            },
            "get_total_transaction_count" => web3_service.get_total_transaction_count().await,
            "call" => {
                match valid_args_len(args_length, 3) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        let tokens = parse_contract_function_tokens(args, &self.config);
                        let response= web3_service.call(
                            &args[0],
                            &args[1],
                            &args[2],
                            &tokens,
                        ).await;
                        match response {
                            Err(err) => Err(err),
                            Ok(data) => {
                                println!("\n{:?}\n", data);
                                Ok(json!(null))
                            }
                        }
                    },
                }
            },
            "send_raw_transaction" => {
                match valid_args_len(args_length, 3) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        let tokens = parse_contract_function_tokens(args, &self.config);
                        web3_service.send_raw_transaction(
                            &args[0],
                            &args[1],
                            &args[2],
                            &tokens,
                        ).await.map(|v| json!(v))
                    },
                }
            },
            "send_raw_transaction_and_get_proof" => {
                match valid_args_len(args_length, 3) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        let tokens = parse_contract_function_tokens(args, &self.config);
                        web3_service.send_raw_transaction_and_get_proof(
                            &args[0],
                            &args[1],
                            &args[2],
                            &tokens,
                        ).await.map(|v| json!(v))
                    },
                }
            },
            "deploy" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        let tokens = parse_contract_constructor_tokens(args, &self.config);
                        web3_service.deploy(&args[0], &tokens).await
                    }
                }
            },
            "compile" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        let link_libraries: Option<HashMap<String, String>> = if args_length > 1 {
                            Some(fisco_bcos_service::serde_json::from_str::<HashMap<String, String>>(&args[1]).unwrap())
                        } else {
                            None
                        };
                        web3_service.compile(&args[0], &link_libraries).await.map(|_| json!(null))
                    }
                }
            },
            "get_system_config_by_key" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_system_config_by_key(&args[0]).await.map(|v| json!(v)),
                }
            },
            "get_transaction_by_hash_with_proof" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_transaction_by_hash_with_proof(&args[0]).await,
                }
            },
            "get_transaction_receipt_by_hash_with_proof" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => web3_service.get_transaction_receipt_by_hash_with_proof(&args[0]).await,
                }
            },
            "generate_group" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        let params = convert_str_to_json(&args[0]);
                        web3_service.generate_group(&params).await
                    },
                }
            },
            "start_group" => web3_service.start_group().await,
            "stop_group" => web3_service.stop_group().await,
            "remove_group" => web3_service.remove_group().await,
            "recover_group" => web3_service.recover_group().await,
            "query_group_status" => web3_service.query_group_status().await,
            "get_node_info" => web3_service.get_node_info().await,
            "get_batch_receipts_by_block_number_and_range" => {
                match valid_args_len(args_length, 4) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        web3_service.get_batch_receipts_by_block_number_and_range(
                            &args[0],
                            convert_str_to_number::<u32>(&args[1], 0),
                            convert_str_to_number::<i32>(&args[2], -1),
                            convert_str_to_bool(&args[3]),
                        ).await
                    },
                }
            },
            "get_batch_receipts_by_block_hash_and_range" => {
                match valid_args_len(args_length, 4) {
                    Err(err) => Err(err),
                    Ok(_) => {
                        web3_service.get_batch_receipts_by_block_hash_and_range(
                            &args[0],
                            convert_str_to_number::<u32>(&args[1], 0),
                            convert_str_to_number::<i32>(&args[2], -1),
                            convert_str_to_bool(&args[3]),
                        ).await
                    },
                }
            },
            command => Err(Web3ServiceError::CustomError {
                message: format!("Unavailable command {:?}", command),
            })
        };
        match response {
            Ok(data) => {
                if !data.is_null() {
                    println!("\n{:?}\n", data)
                }
            },
            Err(error) => println!("\nError: {:?}\n", error),
        };
    }

    async fn call_system_config_service(&self, args: &Vec<String>) {
        let args_length = args.len();
        match valid_args_len(args_length, 2) {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(_) => {
                let web3_service = self.web3_service.as_ref().unwrap();
                let system_config_service = SystemConfigService::new(web3_service);
                let response = system_config_service.set_value_by_key(&args[0], &args[1]).await;
                match response {
                    Err(error) => println!("\nError: {:?}\n", error),
                    Ok(data) =>  println!("\n{:?}\n", data),
                };
            }
        };
    }

    async fn call_consensus_service(&self, method: &str, args: &Vec<String>) {
        let args_length = args.len();
        match valid_args_len(args_length, 1) {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(_) => {
                let web3_service = self.web3_service.as_ref().unwrap();
                let consensus_service = ConsensusService::new(web3_service);
                let response = match method {
                    "consensus:add_sealer" => consensus_service.add_sealer(&args[0]).await,
                    "consensus:add_observer" => consensus_service.add_observer(&args[0]).await,
                    "consensus:remove" => consensus_service.remove(&args[0]).await,
                    command => Err(PrecompiledServiceError::CustomError {
                        message: format!("Unavailable command {:?}", command),
                    })
                };
                match response {
                    Err(error) => println!("\nError: {:?}\n", error),
                    Ok(data) =>  println!("\n{:?}\n", data),
                };
            }
        };
    }

    async fn call_cns_service(&self, method: &str, args: &Vec<String>) {
        let args_length = args.len();
        let web3_service = self.web3_service.as_ref().unwrap();
        let cns_service = CNSService::new(web3_service);
        let response = match method {
            "cns:insert" => {
                match valid_args_len(args_length, 4) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => cns_service.insert(&args[0], &args[1], &args[2], &args[3]).await.map(|v| json!(v)),
                }
            },
            "cns:select_by_name" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => cns_service.select_by_name(&args[0]).await,
                }
            },
            "cns:select_by_name_and_version" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => cns_service.select_by_name_and_version(&args[0], &args[1]).await,
                }
            },
            "cns:get_contract_address" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => cns_service.get_contract_address(&args[0], &args[1]).await.map(|v| json!(v)),
                }
            },
            command => Err(PrecompiledServiceError::CustomError {
                message: format!("Unavailable command {:?}", command),
            })
        };
        match response {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(data) =>  println!("\n{:?}\n", data),
        };
    }

    async fn call_permission_service(&self, method: &str, args: &Vec<String>) {
        let args_length = args.len();
        let web3_service = self.web3_service.as_ref().unwrap();
        let permission_service = PermissionService::new(web3_service);
        let response = match method {
            "permission:insert" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => permission_service.insert(&args[0], &args[1]).await.map(|v| json!(v)),
                }
            },
            "permission:remove" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => permission_service.remove(&args[0], &args[1]).await.map(|v| json!(v)),
                }
            },
            "permission:query_by_name" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => permission_service.query_by_name(&args[0]).await,
                }
            },
            "permission:grant_write" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => permission_service.grant_write(&args[0], &args[1]).await.map(|v| json!(v)),
                }
            },
            "permission:revoke_write" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => permission_service.revoke_write(&args[0], &args[1]).await.map(|v| json!(v)),
                }
            },
            "permission:query_permission" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => permission_service.query_permission(&args[0]).await,
                }
            },
            command => Err(PrecompiledServiceError::CustomError {
                message: format!("Unavailable command {:?}", command),
            })
        };
        match response {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(data) =>  println!("\n{:?}\n", data),
        };
    }

    async fn call_contract_life_cycle_service(&self, method: &str, args: &Vec<String>) {
        let args_length = args.len();
        let web3_service = self.web3_service.as_ref().unwrap();
        let contract_life_cycle_service = ContractLifeCycleService::new(web3_service);
        let response = match method {
            "contract_life_cycle:freeze" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => contract_life_cycle_service.freeze(&args[0]).await.map(|v| json!(v)),
                }
            },
            "contract_life_cycle:unfreeze" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => contract_life_cycle_service.unfreeze(&args[0]).await.map(|v| json!(v)),
                }
            },
            "contract_life_cycle:grant_manager" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => contract_life_cycle_service.grant_manager(&args[0], &args[1]).await.map(|v| json!(v)),
                }
            },
            "contract_life_cycle:get_status" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => contract_life_cycle_service.get_status(&args[0]).await.map(|v| json!(v)),
                }
            },
            "contract_life_cycle:list_manager" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => contract_life_cycle_service.list_manager(&args[0]).await.map(|v| json!(v)),
                }
            },
            command => Err(PrecompiledServiceError::CustomError {
                message: format!("Unavailable command {:?}", command),
            })
        };
        match response {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(data) =>  println!("\n{:?}\n", data),
        };
    }

    async fn call_chain_governance_service(&self, method: &str, args: &Vec<String>) {
        let args_length = args.len();
        let web3_service = self.web3_service.as_ref().unwrap();
        let chain_governance_service = ChainGovernanceService::new(web3_service);
        let response = match method {
            "chain_governance_service:grant_committee_member" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.grant_committee_member(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:revoke_committee_member" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.revoke_committee_member(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:list_committee_members" => chain_governance_service.list_committee_members().await,
            "chain_governance_service:query_committee_member_weight" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.query_committee_member_weight(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:update_committee_member_weight" => {
                match valid_args_len(args_length, 2) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.update_committee_member_weight(
                        &args[0],
                        convert_str_to_number::<i32>(&args[1], 1),
                    ).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:query_votes_of_member" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.query_votes_of_member(&args[0]).await,
                }
            },
            "chain_governance_service:query_votes_of_threshold" => chain_governance_service.query_votes_of_threshold().await,
            "chain_governance_service:update_threshold" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.update_threshold(
                        convert_str_to_number::<i32>(&args[0], 0),
                    ).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:query_threshold" => chain_governance_service.query_threshold().await.map(|v| json!(v)),
            "chain_governance_service:grant_operator" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.grant_operator(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:revoke_operator" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.revoke_operator(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:list_operators" => chain_governance_service.list_operators().await,
            "chain_governance_service:freeze_account" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.freeze_account(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:unfreeze_account" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.unfreeze_account(&args[0]).await.map(|v| json!(v)),
                }
            },
            "chain_governance_service:get_account_status" => {
                match valid_args_len(args_length, 1) {
                    Err(err) => Err(PrecompiledServiceError::Web3ServiceError(err)),
                    Ok(_) => chain_governance_service.get_account_status(&args[0]).await.map(|v| json!(v)),
                }
            },
            command => Err(PrecompiledServiceError::CustomError {
                message: format!("Unavailable command {:?}", command),
            })
        };
        match response {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(data) =>  println!("\n{:?}\n", data),
        };
    }

    async fn call_sql_service(&self, args: &Vec<String>) {
        let args_length = args.len();
        match valid_args_len(args_length, 1) {
            Err(error) => println!("\nError: {:?}\n", error),
            Ok(_) => {
                let web3_service = self.web3_service.as_ref().unwrap();
                let sql_service = SQLService::new(web3_service);
                let response = sql_service.execute(&args[0]).await;
                match response {
                    Err(error) => println!("\nError: {:?}\n", error),
                    Ok(data) =>  println!("\n{:?}\n", data),
                };
            }
        };
    }

    fn echo_help(&self) {
        println!("\n1. Use set_config function to initialize the environment(e.g., set_config ./config/config.json)");
        println!(
            "2. Use the below functions to interact with the FISCO BCOS Service: {:?}(e.g., get_block_by_number 0x0)",
            vec![
                "get_client_version", "get_block_number", "get_pbft_view",
                "get_sealer_list", "get_observer_list", "get_consensus_status",
                "get_sync_status", "get_peers", "get_group_peers",
                "get_node_id_list", "get_group_list", "get_block_by_hash",
                "get_block_by_number", "get_block_header_by_hash", "get_block_header_by_number",
                "get_block_hash_by_number", "get_transaction_by_hash", "get_transaction_by_block_hash_and_index",
                "get_transaction_by_block_number_and_index", "get_transaction_receipt", "get_pending_transactions",
                "get_pending_tx_size", "get_code", "get_total_transaction_count",
                "call", "send_raw_transaction", "send_raw_transaction_and_get_proof", "deploy", "compile",
                "get_system_config_by_key", "get_transaction_by_hash_with_proof", "get_transaction_receipt_by_hash_with_proof",
                "generate_group", "start_group", "stop_group",
                "remove_group", "recover_group", "query_group_status",
                "get_node_info", "get_batch_receipts_by_block_number_and_range", "get_batch_receipts_by_block_hash_and_range",
                "system_config:set_value_key",
                "consensus:add_sealer", "consensus:add_observer", "consensus:remove",
                "cns:insert", "cns:select_by_name", "cns:select_by_name_and_version", "cns:get_contract_address",
                "permission:insert", "permission:remove", "permission:query_by_name",
                "permission:grant_write", "permission:revoke_write", "permission:query_permission",
                "contract_life_cycle:freeze", "contract_life_cycle:unfreeze", "contract_life_cycle:grant_manager",
                "contract_life_cycle:get_status", "contract_life_cycle:list_manager",
                "chain_governance_service:grant_committee_member", "chain_governance_service:revoke_committee_member",
                "chain_governance_service:list_committee_members", "chain_governance_service:query_committee_member_weight",
                "chain_governance_service:update_committee_member_weight",
                "chain_governance_service:query_votes_of_member", "chain_governance_service:query_votes_of_threshold",
                "chain_governance_service:update_threshold", "chain_governance_service:query_threshold",
                "chain_governance_service:grant_operator", "chain_governance_service:revoke_operator", "chain_governance_service:list_operators",
                "chain_governance_service:freeze_account", "chain_governance_service:unfreeze_account", "chain_governance_service:get_account_status",
                "sql",
            ],
        );
        println!("3. Type help to get help");
        println!("4. Type CTRL-C or CTRL-D to quit");
        println!("5. Visit https://github.com/kkawakam/rustyline#actions to get more actions\n");
    }

    pub(crate) fn new() -> Cli {
        Cli { config: None, web3_service: None }
    }

    pub(crate) async fn run_command(&mut self, command: &str) {
        let re = fancy_regex::Regex::new(r#"(".+"|'.+'|[^\s]+)"#).unwrap();
        let parts: Vec<&str> = re.find_iter(command)
            .map(|item| item.unwrap().as_str().trim_start_matches("'").trim_end_matches("'"))
            .collect();
        let method = parts[0];
        let args: Vec<String> = if parts.len() > 1 {
            parts[1..].iter().map(|&v| v.to_owned()).collect()
        } else {
            vec![]
        };
        let args_length = args.len();
        match method {
            "help" => self.echo_help(),
            "set_config" => {
                match valid_args_len(args_length, 1) {
                    Ok(_) => self.set_config(&args[0]),
                    Err(error) => println!("\nError: {:?}\n", error),
                }
            },
            _ => {
                if self.web3_service.is_none() {
                    println!("\nError: Please initialize the environment with set_config function first\n");
                } else {
                    if method.starts_with("system_config:") {
                        self.call_system_config_service(&args).await
                    } else if method.starts_with("consensus:") {
                        self.call_consensus_service(method, &args).await
                    } else if method.starts_with("cns:") {
                        self.call_cns_service(method, &args).await
                    } else if method.starts_with("permission:") {
                        self.call_permission_service(method, &args).await
                    } else if method.starts_with("contract_life_cycle:") {
                        self.call_contract_life_cycle_service(method, &args).await
                    } else if method.starts_with("chain_governance_service:")  {
                        self.call_chain_governance_service(method, &args).await
                    } else if method.eq("sql") {
                        self.call_sql_service(&args).await
                    } else {
                        self.call_web3_service(method, &args).await
                    }
                }
            },
        };
    }
}