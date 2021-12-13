use serde_json::{json, Value as JSONValue};
use sqlparser::ast::{ColumnDef, ObjectName, Statement, TableConstraint};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::{
    precompiled_service::PrecompiledServiceError,
    table_factory_service::TableFactoryService,
};

pub struct SQLService<'a> {
    web3_service: &'a Web3Service,
}

impl SQLService<'_> {
    async fn execute_create_table(
        &self,
        name: &ObjectName,
        columns: &Vec<ColumnDef>,
        constraints: &Vec<TableConstraint>
    ) -> Result<i32, PrecompiledServiceError> {
        let mut primary_keys: Vec<String> = vec![];
        for constraint in constraints {
            primary_keys.extend(match constraint {
                TableConstraint::Unique { is_primary, columns, .. } => {
                    if *is_primary {
                        columns.into_iter().map(|column| column.clone().value).collect()
                    } else {
                        vec![]
                    }
                },
                _ => vec![],
            });
        }
        if primary_keys.len() == 0 {
            return Err(PrecompiledServiceError::CustomError {
                message: String::from("No primary key specified"),
            });
        }

        if primary_keys.len() > 1 {
            return Err(PrecompiledServiceError::CustomError {
                message: String::from("Primary key specified more than once"),
            });
        }

        let table_name: String = (&name.0)[0].clone().value;
        let fields: Vec<String> = columns.into_iter().map(|column| column.name.clone().value)
            .filter(|field| !primary_keys.contains(field))
            .collect();
        let table_factory_service = TableFactoryService::new(self.web3_service);
        Ok(table_factory_service.create_table(&table_name, &primary_keys[0], &fields).await?)
    }

    pub fn new(web3_service: &Web3Service) -> SQLService {
        SQLService {
            web3_service
        }
    }

    pub async fn execute(&self, sql: &str) -> Result<JSONValue, PrecompiledServiceError> {
        let dialect = GenericDialect {};
        let ast: Vec<Statement> = Parser::parse_sql(&dialect, sql)?;
        match &ast[0] {
            Statement::CreateTable { name, columns, constraints, .. } => {
                Ok(json!(self.execute_create_table(name, columns, constraints).await?))
            },
            _ => Err(PrecompiledServiceError::CustomError {
                message: format!("Invalid sql:{:?}", sql),
            })
        }
    }
}