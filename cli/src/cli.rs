use std::{
    str::FromStr,
    collections::HashMap,
};
use fisco_bcos_service::{
    abi::ABI,
    config::Config,
    ethabi::token::Token,
    serde_json::{
        json,
        Value as JSONValue,
    },
    web3::service::{
        Service as Web3Service,
        ServiceError as Web3ServiceError,
    },
    precompiled::{
        cns_service::CNSService,
        sql_service::SQLService,
        consensus_service::ConsensusService,
        permission_service::PermissionService,
        system_config_service::SystemConfigService,
        chain_governance_service::ChainGovernanceService,
        contract_life_cycle_service::ContractLifeCycleService,
        precompiled_service::PrecompiledServiceError,
    },
    create_config_with_file,
    create_web3_service_with_config,
};

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
        match create_config_with_file(config_path) {
            Ok(config) => {
                match create_web3_service_with_config(&config) {
                    Ok(web3_service) => {
                        self.web3_service = Some(web3_service);
                    },
                    Err(error) =>  println!("\n Web3 Service initialize error: {:?}\n", error),
                };
                self.config = Some(config);
            },
            Err(error) => println!("\n Config initialize error: {:?}\n", error),
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
        println!("\n1. Use set_config to initialize environment(e.g., set_config ./config/config.json).");
        println!("2. Use the below APIs to interact with FISCO BCOS：\n");
        println!("* get_client_version                                         Query the current node version.");
        println!("* get_block_number                                           Query the number of most recent block.");
        println!("* get_pbft_view                                              Query the pbft view of node.");
        println!("* get_sealer_list                                            Query nodeId list for sealer nodes.");
        println!("* get_observer_list                                          Query nodeId list for observer nodes.");
        println!("* get_consensus_status                                       Query consensus status.");
        println!("* get_sync_status                                            Query sync status.");
        println!("* get_peers                                                  Query peers currently connected to the client.");
        println!("* get_group_peers                                            Query nodeId list for sealer and observer nodes.");
        println!("* get_node_id_list                                           Query nodeId list for all connected nodes.");
        println!("* get_group_list                                             Query group list.");
        println!("* get_block_by_hash                                          Query information about a block by hash.");
        println!("* get_block_by_number                                        Query information about a block by number.");
        println!("* get_block_header_by_hash                                   Query information about a block header by hash.");
        println!("* get_block_header_by_number                                 Query information about a block header by block number.");
        println!("* get_block_hash_by_number                                   Query block hash by block number.");
        println!("* get_transaction_by_hash                                    Query information about a transaction requested by transaction hash.");
        println!("* get_transaction_by_block_hash_and_index                    Query information about a transaction by block hash and transaction index position.");
        println!("* get_transaction_by_block_number_and_index                  Query information about a transaction by block number and transaction index position.");
        println!("* get_transaction_receipt                                    Query the receipt of a transaction by transaction hash.");
        println!("* get_pending_transactions                                   Query pending transactions.");
        println!("* get_pending_tx_size                                        Query pending transactions size.");
        println!("* get_code                                                   Query code at a given address.");
        println!("* get_total_transaction_count                                Query total transaction count.");
        println!("* get_system_config_by_key                                   Query a system config value by key.");
        println!("* call                                                       Call a contract by a function and parameters.");
        println!("* send_raw_transaction                                       Execute a signed transaction with a contract function and parameters.");
        println!("* send_raw_transaction_and_get_proof                         Execute a signed transaction with a contract function and parameters.");
        println!("* deploy                                                     Deploy a contract on blockchain.");
        println!("* compile                                                    Compile sol file to abi & bin files.");
        println!("* get_transaction_by_hash_with_proof                         Query the transaction and transaction proof by transaction hash.");
        println!("* get_transaction_receipt_by_hash_with_proof                 Query the receipt and transaction receipt proof by transaction hash.");
        println!("* generate_group                                             Generate a group for the specified node.");
        println!("* start_group                                                Start the specified group of the specified node.");
        println!("* stop_group                                                 Stop the specified group of the specified node.");
        println!("* remove_group                                               Remove the specified group of the specified node.");
        println!("* recover_group                                              Recover the specified group of the specified node.");
        println!("* query_group_status                                         Query the status of the specified group of the specified node.");
        println!("* get_node_info                                              Query the specified node information.");
        println!("* get_batch_receipts_by_block_number_and_range               Get batched transaction receipts according to block number and the transaction range.");
        println!("* get_batch_receipts_by_block_hash_and_range                 Get batched transaction receipts according to block hash and the transaction range.");
        println!("* system_config:set_value_by_key                             SystemConfigPrecompiled: Set a system config value by key.");
        println!("* consensus:add_sealer                                       ConsensusPrecompiled: Add a sealer node.");
        println!("* consensus:add_observer                                     ConsensusPrecompiled: Add an observer node.");
        println!("* consensus:remove                                           ConsensusPrecompiled: Remove a node.");
        println!("* cns:insert                                                 CNSPrecompiled: Insert CNS information for the given contract");
        println!("* cns:select_by_name                                         CNSPrecompiled: Query CNS information by contract name.");
        println!("* cns:select_by_name_and_version                             CNSPrecompiled: Query CNS information by contract name and contract version.");
        println!("* cns:get_contract_address                                   CNSPrecompiled: Query contract address by contract name.");
        println!("* permission:insert                                          PermissionPrecompiled: Grant the specified account write permission for the specified table.");
        println!("* permission:remove                                          PermissionPrecompiled: Remove the specified account write permission for the specified table.");
        println!("* permission:query_by_name                                   PermissionPrecompiled: Query the accounts who have write permission for the specified table.");
        println!("* permission:grant_write                                     PermissionPrecompiled: Grant the specified account write permission for the specified contract.");
        println!("* permission:revoke_write                                    PermissionPrecompiled: Revoke the specified account write permission for the specified contract.");
        println!("* permission:query_permission                                PermissionPrecompiled: Query the accounts who have write permission for the specified contract.");
        println!("* contract_life_cycle:freeze                                 ContractLifeCyclePrecompiled: Freeze the specified contract.");
        println!("* contract_life_cycle:unfreeze                               ContractLifeCyclePrecompiled: Unfreeze the specified contract.");
        println!("* contract_life_cycle:grant_manager                          ContractLifeCyclePrecompiled: Authorize a account to be the manager of the contract.");
        println!("* contract_life_cycle:get_status                             ContractLifeCyclePrecompiled: Query the status of the specified contract.");
        println!("* contract_life_cycle:list_manager                           ContractLifeCyclePrecompiled: Query the managers of the specified contract.");
        println!("* chain_governance_service:grant_committee_member            ChainGovernancePrecompiled: Grant the account committee member.");
        println!("* chain_governance_service:revoke_committee_member           ChainGovernancePrecompiled: Revoke the account from committee member.");
        println!("* chain_governance_service:list_committee_members            ChainGovernancePrecompiled: List all committee members.");
        println!("* chain_governance_service:query_committee_member_weight     ChainGovernancePrecompiled: Query the committee member weight.");
        println!("* chain_governance_service:update_committee_member_weight    ChainGovernancePrecompiled: Update the committee member weight.");
        println!("* chain_governance_service:query_votes_of_member             ChainGovernancePrecompiled: Query votes of a committee member.");
        println!("* chain_governance_service:query_votes_of_threshold          ChainGovernancePrecompiled: Query votes of updateThreshold operation.");
        println!("* chain_governance_service:update_threshold                  ChainGovernancePrecompiled: Update the threshold.");
        println!("* chain_governance_service:query_threshold                   ChainGovernancePrecompiled: Query the threshold.");
        println!("* chain_governance_service:grant_operator                    ChainGovernancePrecompiled: Grant the operator.");
        println!("* chain_governance_service:revoke_operator                   ChainGovernancePrecompiled: Revoke the operator.");
        println!("* chain_governance_service:list_operators                    ChainGovernancePrecompiled: List all operators.");
        println!("* chain_governance_service:freeze_account                    ChainGovernancePrecompiled: Freeze the contract");
        println!("* chain_governance_service:unfreeze_account                  ChainGovernancePrecompiled: Unfreeze the contract.");
        println!("* chain_governance_service:get_account_status                ChainGovernancePrecompiled: Get the contract status.");
        println!("* sql                                                        Execute CRUD operations with SQL.\n");
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
                if self.config.is_none() {
                    println!("\nError: Please initialize the environment with set_config first\n");
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