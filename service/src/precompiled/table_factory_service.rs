use crate::web3::service::Service as Web3Service;
use crate::precompiled::precompiled_service::{
    PrecompiledServiceError,
    call,
    send_transaction,
    parse_address_token_to_string,
};

const ADDRESS: &str = "0x0000000000000000000000000000000000001001";
const ABI_CONTENT: &str = r#"[{"constant":false,"inputs":[{"name":"","type":"string"},{"name":"","type":"string"},{"name":"","type":"string"}],"name":"createTable","outputs":[{"name":"","type":"int256"}],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"","type":"string"}],"name":"openTable","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"}]"#;

pub struct TableFactoryService<'a> {
    web3_service: &'a Web3Service,
}

impl TableFactoryService<'_> {
    pub fn new(web3_service: &Web3Service) -> TableFactoryService {
        TableFactoryService {
            web3_service
        }
    }

    pub async fn open_table(&self, table_name: &str) -> Result<String, PrecompiledServiceError> {
        let params = vec![table_name.to_owned()];
        let response = call(
            self.web3_service,
            "TableFactory",
            ADDRESS,
            ABI_CONTENT,
            "openTable",
            &params
        ).await?;
        parse_address_token_to_string(&response.output)
    }

    pub async fn create_table(&self, table_name: &str, key_field: &str, fields: &Vec<String>) -> Result<i32, PrecompiledServiceError> {
        if fields.len() == 0 {
            return  Err(PrecompiledServiceError::CustomError {
                message: String::from("The size of the field must be larger than 0"),
            });
        }

        if fields.contains(&key_field.to_owned()) {
            return Err(PrecompiledServiceError::CustomError {
                message: format!("The fields should not include the key field {:?}", key_field),
            });
        }

        let params = vec![table_name.to_owned(), key_field.to_owned(), fields.join(",")];
        send_transaction(
            self.web3_service,
            "TableFactory",
            ADDRESS,
            ABI_CONTENT,
            "createTable",
            &params
        ).await
    }
}