use std::fs;
use std::collections::HashMap;
use std::process::Command;
use thiserror::Error;
use wedpr_l_utils::traits::Hash;
use wedpr_l_crypto_hash_sm3::WedprSm3;
use ethabi::{
    Contract, Bytes, Function, Param,
    param_type::{ParamType, Writer},
    token::{StrictTokenizer, Token, Tokenizer},
    Error as ETHError,
    Result as ETHResult,
    decode as eth_decode,
    encode as eth_encode,
};

use crate::config::Contract as ContractConfig;

#[derive(Error, Debug)]
pub enum ABIError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("ethabi::Error")]
    ETHABIError(#[from] ETHError),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),

    #[error("abi custom error")]
    CustomError {
        message: String,
    }
}

pub struct ABI {
    contract: Option<Contract>,
    contract_name: String,
    contract_config: ContractConfig,
    sm_crypto: bool,
}

impl ABI {
    fn get_load_contract_error(&self) -> ABIError {
        ABIError::CustomError {
            message: format!("Can't load the contract:{:?}, please compile it first", self.contract_name)
        }
    }

    fn parse_tokens(&self, inputs: &Vec<Param>, params: &Vec<String>) -> Result<Vec<Token>, ABIError> {
        let params: Vec<(ParamType, &str)> = inputs.iter()
            .map(|param| param.kind.clone())
            .zip(params.iter().map(|v| v as &str)).collect();
        let tokens = params.iter()
            .map(|&(ref param, value)| StrictTokenizer::tokenize(param, value))
            .collect::<Result<Vec<Token>, ETHError>>()?;
        Ok(tokens)
    }

    fn sm_short_signature(&self, name: &str, params: &[ParamType]) -> Vec<u8> {
        let types = params.iter().map(Writer::write).collect::<Vec<String>>().join(",");
        let data: Vec<u8> = From::from(format!("{}({})", name, types).as_str());
        let sm3_hash = WedprSm3::default();
        sm3_hash.hash(&data)[..4].to_vec()
    }

    fn encode_sm_input(&self, function: &Function, tokens: &Vec<Token>) -> ETHResult<Bytes> {
        let params: Vec<ParamType> = function.inputs.iter().map(|p| p.kind.clone()).collect();
        if !Token::types_check(tokens, &params) {
            return Err(ETHError::InvalidData);
        }
        let mut transaction_data = self.sm_short_signature(&function.name, &params);
        transaction_data.extend(eth_encode(tokens));
        Ok(transaction_data)
    }

    pub fn new(contract_config: &ContractConfig, contract_name: &str, sm_crypto: bool) -> Result<ABI, ABIError> {
        let abi_path = contract_config.get_abi_path(contract_name);
        Ok(
            ABI {
                sm_crypto,
                contract_name: contract_name.to_owned(),
                contract_config: contract_config.clone(),
                contract: if abi_path.is_file() {
                    Some(Contract::load(fs::File::open(abi_path)?)?)
                } else {
                    None
                }
            }
        )
    }

    ///
    /// link_libraries 中的键为要链接的 library 的名称，其值为要链接的 library 的地址
    ///
    pub fn compile(&mut self, link_libraries: &Option<HashMap<String, String>>) -> Result<(), ABIError> {
        let contract_path = self.contract_config.get_contract_path(&self.contract_name);
        if !contract_path.is_file() {
            return Err(
                ABIError::CustomError {
                    message: format!("Can't find the contract:{:?}", self.contract_name)
                }
            );
        }

        let mut compile_command = Command::new(&self.contract_config.solc);
        let link_libraries: Vec<String> = match link_libraries {
            None => vec![],
            Some(link_libraries) => {
                let mut result: Vec<String> = vec![];
                for (name, address) in link_libraries  {
                    result.push(format!("{:?}.sol:{:?}:{:?}", self.contract_name, name, address))
                }
                result
            }
        };
        if link_libraries.len() > 0 {
            compile_command.arg("--libraries");
            compile_command.arg(link_libraries.join(" "));
        }
        compile_command.arg("--overwrite").arg("--abi").arg("--bin").arg("-o");
        compile_command.arg(self.contract_config.output.clone());
        compile_command.arg(contract_path);
        let status = compile_command.status()?;
        if status.success() {
            self.contract = Some(Contract::load(fs::File::open(self.contract_config.get_abi_path(&self.contract_name))?)?);
            Ok(())
        } else {
            Err(
                ABIError::CustomError {
                    message: format!("Can't compile the contract:{:?}, please try it again later", self.contract_name)
                }
            )
        }
    }

    pub fn parse_function_tokens(&self, function_name: &str, params: &Vec<String>) -> Result<Vec<Token>, ABIError> {
        match self.contract.as_ref() {
            None => Err(self.get_load_contract_error()),
            Some(contract) => {
                let function = contract.function(&function_name)?;
                Ok(self.parse_tokens(&function.inputs, params)?)
            }
        }
    }

    pub fn parse_constructor_tokens(&self, params: &Vec<String>) -> Result<Vec<Token>, ABIError> {
        match self.contract.as_ref() {
            None => Err(self.get_load_contract_error()),
            Some(contract) => {
                let constructor= contract.constructor.as_ref().unwrap();
                Ok(self.parse_tokens(&constructor.inputs, params)?)
            }
        }
    }

    pub fn encode_constructor_input(&self, tokens: &Vec<Token>) -> Result<Vec<u8>, ABIError> {
        match self.contract.as_ref() {
            None => Err(self.get_load_contract_error()),
            Some(contract) => {
                let constructor= contract.constructor.as_ref().unwrap();
                let abi_bin_path = self.contract_config.get_abi_bin_path(&self.contract_name);
                Ok(hex::decode(Vec::from(constructor.encode_input(fs::read(&abi_bin_path)?, &tokens)?))?)
            }
        }
    }

    pub fn encode_function_input(&self, function_name: &str, tokens: &Vec<Token>) -> Result<Vec<u8>, ABIError> {
        match self.contract.as_ref() {
            None => Err(self.get_load_contract_error()),
            Some(contract) => {
                let function = contract.function(&function_name)?;
                if self.sm_crypto {
                    Ok(self.encode_sm_input(&function, &tokens)?)
                } else {
                    Ok(function.encode_input(&tokens)?)
                }
            }
        }
    }

    pub fn decode_output(&self, function_name: &str, value: &str) -> Result<Option<Vec<Token>>, ABIError> {
        if value.eq("0x") {
            return Ok(None);
        }
        match self.contract.as_ref() {
            None => Err(self.get_load_contract_error()),
            Some(contract) => {
                let function = contract.function(&function_name)?;
                if value.starts_with("0x08c379a0") {
                    let output = &value[10..];
                    let data = hex::decode(output.to_owned().as_bytes())?;
                    let params: Vec<ParamType> = vec![ParamType::String];
                    let tokens = eth_decode(&params, &data)?;
                    return Err(ABIError::CustomError { message: tokens[0].to_string() });
                }
                let data = hex::decode(value.to_owned().trim_start_matches("0x").as_bytes())?;
                Ok(Some(function.decode_output(&data)?))
            }
        }
    }
}