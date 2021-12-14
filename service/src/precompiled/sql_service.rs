use std::collections::HashMap;
use serde_json::{json, Value as JSONValue};
use sqlparser::ast::{ColumnDef, Expr, Ident, ObjectName, SetExpr, Statement, TableConstraint};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::{
    crud_service::CRUDService,
    table_factory_service::TableFactoryService,
    precompiled_service::PrecompiledServiceError,
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
        let mut key_fields: Vec<String> = vec![];
        for constraint in constraints {
            key_fields.extend(match constraint {
                TableConstraint::Unique { is_primary, columns, .. } => {
                    if *is_primary {
                        columns.into_iter().map(|column| column.value.clone()).collect()
                    } else {
                        vec![]
                    }
                },
                _ => vec![],
            });
        }
        if key_fields.len() == 0 {
            return Err(PrecompiledServiceError::CustomError {
                message: String::from("No key field specified"),
            });
        }

        if key_fields.len() > 1 {
            return Err(PrecompiledServiceError::CustomError {
                message: String::from("Key field specified more than once"),
            });
        }

        let table_name: String = (&name.0)[0].value.clone();
        let fields: Vec<String> = columns.into_iter().map(|column| column.name.value.clone())
            .filter(|field| !key_fields.contains(field))
            .collect();
        let table_factory_service = TableFactoryService::new(self.web3_service);
        Ok(table_factory_service.create_table(&table_name, &key_fields[0], &fields).await?)
    }

    async fn fetch_table_fields(&self, table_name: &str) -> Result<(String, Vec<String>), PrecompiledServiceError> {
        let crud_service = CRUDService::new(self.web3_service);
        let (key_field, value_fields) = crud_service.desc(&table_name).await?;
        let mut table_fields: Vec<String> = vec![];
        if key_field.len() > 0 {
            table_fields.push(key_field.clone());
        }
        if value_fields.len() > 0 {
            table_fields.extend(value_fields);
        }
        if table_fields.is_empty() {
            Err(PrecompiledServiceError::CustomError {
                message: format!("Can't fetch fields for table {:?}", table_name),
            })
        } else {
            Ok((key_field, table_fields))
        }
    }

    async fn execute_insert(
        &self,
        name: &ObjectName,
        columns: &Vec<Ident>,
        values: &Vec<Expr>,
    ) -> Result<i32, PrecompiledServiceError> {
        let table_name: String = (&name.0)[0].value.clone();
        let (key_field, table_fields) = self.fetch_table_fields(&table_name).await?;
        let value_fields = if columns.len() > 0 {
            columns.into_iter().map(|column| column.value.clone()).collect()
        } else {
            table_fields.clone()
        };
        let invalid_fields: Vec<String> = value_fields.clone()
            .into_iter()
            .filter(|field| !table_fields.contains(field))
            .collect();
        if invalid_fields.len() > 0 {
            return Err(PrecompiledServiceError::CustomError {
                message: format!("Invalid fields {:?} for table {:?}", invalid_fields.join(","), table_name),
            });
        }
        let value_length = values.len();
        let value_field_length = value_fields.len();
        if value_length != value_field_length {
            return Err(PrecompiledServiceError::CustomError {
                message: format!("Unmatched number of values, expected {:?} but got {:?}", value_field_length, value_length),
            });
        }

        let mut key_field_value: String = String::from("");
        let mut entry: HashMap<String, String> = HashMap::new();
        for (index, value_field) in value_fields.into_iter().enumerate() {
            if let Expr::Identifier(ident) = &values[index] {
                if value_field.eq(&key_field) {
                    key_field_value = ident.value.clone();
                } else {
                    entry.insert(value_field, ident.value.clone());
                }
            }
        }
        if key_field_value.is_empty() {
            Err(PrecompiledServiceError::CustomError {
                message: format!("Value of key field {:?} should be provided", key_field),
            })
        } else {
            let crud_service = CRUDService::new(self.web3_service);
            Ok(crud_service.insert(&table_name, &key_field_value, &entry).await?)
        }
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
            Statement::Insert { table_name, columns, source, .. } => {
                if let SetExpr::Values(values) = &source.body {
                    Ok(json!(self.execute_insert(table_name, columns, &(values.0)[0]).await?))
                } else {
                    Err(PrecompiledServiceError::CustomError {
                        message: format!("Invalid sql {:?}", sql),
                    })
                }
            },
            _ => Err(PrecompiledServiceError::CustomError {
                message: format!("Invalid sql:{:?}", sql),
            })
        }
    }
}