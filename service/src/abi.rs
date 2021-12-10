use std::fs;
use thiserror::Error;
use wedpr_l_utils::traits::Hash;
use wedpr_l_crypto_hash_sm3::WedprSm3;
use ethabi::{
    Bytes, Contract, Function,
    Param, param_type::{ParamType, Writer},
    token::{LenientTokenizer, Token, Tokenizer},
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
    sm_crypto: bool,
    contract_name: String,
    abi_bin: Option<Vec<u8>>,
    contract: Option<Contract>,
}

impl ABI {
    fn get_load_contract_error(&self) -> ABIError {
        ABIError::CustomError {
            message: format!("Can't load the contract:{:?}, please compile it first", self.contract_name)
        }
    }

    fn get_load_abi_bin_error(&self) -> ABIError {
        ABIError::CustomError {
            message: format!("Can't load abi bin for the contract:{:?}, please set it first", self.contract_name)
        }
    }

    fn parse_tokens(&self, inputs: &Vec<Param>, params: &Vec<String>) -> Result<Vec<Token>, ABIError> {
        let params: Vec<(ParamType, &str)> = inputs.iter()
            .map(|param| param.kind.clone())
            .zip(params.iter().map(|v| v as &str)).collect();
        let tokens = params.iter()
            .map(|&(ref param, value)| {
                if *param == ParamType::Address {
                    LenientTokenizer::tokenize(param, value.to_owned().trim_start_matches("0x"))
                } else {
                    LenientTokenizer::tokenize(param, value)
                }
            })
            .collect::<Result<Vec<Token>, ETHError>>()?;
        Ok(tokens)
    }

    fn sm_short_signature(&self, name: &str, params: &[ParamType]) -> Vec<u8> {
        let types = params.iter().map(Writer::write).collect::<Vec<String>>().join(",");
        let data: Vec<u8> = From::from(format!("{}({})", name, types).as_str());
        let sm3_hash = WedprSm3::default();
        sm3_hash.hash(&data)[..4].to_vec()
    }

    fn encode_sm_input(&self, function: &Function, tokens: &Vec<Token>) -> ETHResult<Vec<u8>> {
        let params: Vec<ParamType> = function.inputs.iter().map(|p| p.kind.clone()).collect();
        if !Token::types_check(tokens, &params) {
            return Err(ETHError::InvalidData);
        }
        let mut transaction_data = self.sm_short_signature(&function.name, &params);
        transaction_data.extend(eth_encode(tokens));
        Ok(transaction_data)
    }

    pub fn new_with_contract_config(
        contract_config: &ContractConfig,
        contract_name: &str,
        sm_crypto: bool,
    ) -> Result<ABI, ABIError> {
        let abi_path = contract_config.get_abi_path(contract_name);
        let abi = if abi_path.is_file() {
            Some(fs::read(&abi_path)?)
        } else {
            None
        };
        let abi_bin_path = contract_config.get_abi_bin_path(contract_name);
        let abi_bin = if abi_bin_path.is_file() {
            Some(fs::read(&abi_bin_path)?)
        } else {
            None
        };
        Ok(ABI::new(&abi, &abi_bin, contract_name, sm_crypto)?)
    }

    pub fn new(
        abi: &Option<Vec<u8>>,
        abi_bin: &Option<Vec<u8>>,
        contract_name: &str,
        sm_crypto: bool,
    ) -> Result<ABI, ABIError> {
        Ok(
            ABI {
                sm_crypto,
                contract_name: contract_name.to_owned(),
                abi_bin: abi_bin.clone(),
                contract: match abi {
                    None => None,
                    Some(abi) => Some(Contract::load(abi.as_slice())?)
                },
            }
        )
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
                let constructor = contract.constructor.as_ref().unwrap();
                Ok(self.parse_tokens(&constructor.inputs, params)?)
            }
        }
    }

    pub fn encode_constructor_input(&self, tokens: &Vec<Token>) -> Result<Vec<u8>, ABIError> {
        match self.contract.as_ref() {
            None => Err(self.get_load_contract_error()),
            Some(contract) => {
                match self.abi_bin.as_ref() {
                    None => Err(self.get_load_abi_bin_error()),
                    Some(abi_bin) => {
                        let constructor = contract.constructor.as_ref().unwrap();
                        let inputs = constructor.encode_input(Bytes::from(abi_bin.as_slice()), &tokens)?;
                        Ok(hex::decode(Vec::from(inputs))?)
                    },
                }
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