use std::fs;
use thiserror::Error;
use ethabi::{Contract, param_type::{ParamType}, token::{StrictTokenizer, Token, Tokenizer}, Param};

#[derive(Error, Debug)]
pub enum ABIError {
    #[error("std::io::Error")]
    StdIOError(#[from] std::io::Error),

    #[error("ethabi::Error")]
    ETHABIError(#[from] ethabi::Error),

    #[error("hex::FromHexError")]
    FromHexError(#[from] hex::FromHexError),
}

pub struct ABI {
    contract: Contract,
}

impl ABI {
    pub fn new(abi_path: &str) -> Result<ABI, ABIError> {
        let contract = Contract::load(fs::File::open(abi_path)?)?;
        Ok(ABI { contract })
    }

    fn get_tokens_from_inputs(&self, inputs: &Vec<Param>, params: &Vec<String>) -> Result<Vec<Token>, ABIError> {
        let params: Vec<(ParamType, &str)> = inputs.iter()
            .map(|param| param.kind.clone())
            .zip(params.iter().map(|v| v as &str)).collect();
        let tokens = params.iter()
            .map(|&(ref param, value)| StrictTokenizer::tokenize(param, value))
            .collect::<Result<Vec<Token>, ethabi::Error>>()?;
        Ok(tokens)
    }

    pub fn encode_constructor_input(&self, abi_bin_path: &str, params: &Vec<String>) -> Result<Vec<u8>, ABIError> {
        let constructor = self.contract.constructor.as_ref().unwrap();
        let tokens = self.get_tokens_from_inputs(&constructor.inputs, params)?;
        Ok(Vec::from(constructor.encode_input(fs::read(&abi_bin_path)?, &tokens)?))
    }

    pub fn encode_input(&self, function_name: &str, params: &Vec<String>) -> Result<Vec<u8>, ABIError> {
        let function = self.contract.function(&function_name)?;
        let tokens = self.get_tokens_from_inputs(&function.inputs, params)?;
        Ok(Vec::from(function.encode_input(&tokens)?))
    }

    pub fn decode_output(&self, function_name: &str, value: &str) -> Result<Vec<Token>, ABIError> {
        let data = hex::decode(value.to_owned().trim_start_matches("0x").as_bytes())?;
        let function = self.contract.function(&function_name)?;
        Ok(function.decode_output(&data)?)
    }
}