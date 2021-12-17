use std::collections::HashMap;
use serde_json::{json, Value as JSONValue};
use sqlparser::ast::{
    Assignment, BinaryOperator,
    ColumnDef, Expr, Ident, ObjectName,
    Query, SelectItem, SetExpr, Statement,
    TableConstraint, TableFactor, TableWithJoins,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

use crate::web3::service::Service as Web3Service;
use crate::precompiled::{
    precompiled_service::PrecompiledServiceError,
    table_crud_service::{ TableCRUDService, Condition, ConditionOperator },
};

pub struct SQLService<'a> {
    web3_service: &'a Web3Service,
}

impl SQLService<'_> {
    fn get_value_from_expr(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::Identifier(ident) => Some(ident.value.clone()),
            Expr::Value(value) => Some(value.clone().to_string()),
            _ => None,
        }
    }

    fn get_key_field_value_from_condition(
        &self,
        key_field: &str,
        condition: &Condition,
    ) -> Result<String, PrecompiledServiceError> {
        let err = Err(PrecompiledServiceError::CustomError {
            message: format!("Value of key field {:?} should be provided in where clause", key_field),
        });
        match condition.get_condition_by_key(&key_field) {
            Some(v) => match v.get("eq") {
                Some(v) => if v.is_empty() {
                    err
                } else {
                    Ok(v.clone())
                },
                None => err,
            },
            None => err
        }
    }

    fn validate_fields(
        &self,
        table_name: &str,
        table_fields: &Vec<String>,
        expected_fields: &Vec<String>,
    ) -> Result<(), PrecompiledServiceError> {
        let invalid_fields: Vec<String> = expected_fields.clone()
            .into_iter()
            .filter(|field| !table_fields.contains(field))
            .collect();
        if invalid_fields.len() > 0 {
            Err(PrecompiledServiceError::CustomError {
                message: format!("Invalid fields {:?} for table {:?}", invalid_fields.join(","), table_name),
            })
        } else {
            Ok(())
        }
    }

    fn parse_condition(&self, selection: &Expr, condition: &mut Condition) {
        match selection {
            Expr::BinaryOp { left, op, right } => {
                // 因多个 where 条件之间仅支持 AND 操作符且单个 where 条件仅包含 =、!=、>、<、>=、<= 操作符。
                // 故此处可以断定，如果操作符为 AND 时即表示多个 where 条件的连接（比如 name = "Tom" and age = "18"），
                // 否则即为单个 where 条件的解析（比如 name = "Tom"）。
                let left = left.as_ref();
                let right = right.as_ref();
                match op {
                    BinaryOperator::And => {
                        self.parse_condition(left, condition);
                        self.parse_condition(right, condition);
                    },
                    _ => {
                        let key = self.get_value_from_expr(left);
                        let value = self.get_value_from_expr(right);
                        if key.is_some() && value.is_some() {
                            match op {
                                BinaryOperator::Eq => {
                                    condition.set_condition(&key.unwrap(), &value.unwrap(), ConditionOperator::Eq);
                                },
                                BinaryOperator::NotEq => {
                                    condition.set_condition(&key.unwrap(), &value.unwrap(), ConditionOperator::NotEq);
                                },
                                BinaryOperator::Gt => {
                                    condition.set_condition(&key.unwrap(), &value.unwrap(), ConditionOperator::Gt);
                                },
                                BinaryOperator::Lt => {
                                    condition.set_condition(&key.unwrap(), &value.unwrap(), ConditionOperator::Lt);
                                },
                                BinaryOperator::GtEq => {
                                    condition.set_condition(&key.unwrap(), &value.unwrap(), ConditionOperator::GtEq);
                                },
                                BinaryOperator::LtEq => {
                                    condition.set_condition(&key.unwrap(), &value.unwrap(), ConditionOperator::LtEq);
                                },
                                _ => {},
                            };
                        }
                    }
                };
            },
            _ => {},
        }
    }

    fn parse_selection(
        &self,
        table_name: &str,
        key_field: &str,
        table_fields: &Vec<String>,
        selection: &Option<Expr>,
        condition: &mut Condition,
    ) -> Result<String, PrecompiledServiceError> {
        match &selection {
            Some(selection) => self.parse_condition(selection, condition),
            None => {},
        };
        let key_field_value = self.get_key_field_value_from_condition(&key_field, &condition)?;
        let condition_fields = condition.get_condition_keys();
        let _ = self.validate_fields(&table_name, &table_fields, &condition_fields)?;
        Ok(key_field_value)
    }


    async fn fetch_table_fields(&self, table_name: &str) -> Result<(String, Vec<String>), PrecompiledServiceError> {
        let table_crud_service = TableCRUDService::new(self.web3_service);
        let (key_field, value_fields) = table_crud_service.desc(&table_name).await?;
        if key_field.is_empty() {
            return Err(PrecompiledServiceError::CustomError {
                message: format!("Can't fetch key field for table {:?}", table_name),
            });
        }
        let mut table_fields: Vec<String> = vec![];
        table_fields.push(key_field.clone());
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
        let table_crud_service = TableCRUDService::new(self.web3_service);
        Ok(table_crud_service.create_table(&table_name, &key_fields[0], &fields).await?)
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
        let _ = self.validate_fields(&table_name, &table_fields, &value_fields)?;
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
            let value = self.get_value_from_expr(&values[index]).unwrap_or(String::from(""));
            if value_field.eq(&key_field) {
                key_field_value = value;
            } else {
                entry.insert(value_field, value);
            }
        }
        if key_field_value.is_empty() {
            Err(PrecompiledServiceError::CustomError {
                message: format!("Value of key field {:?} should be provided", key_field),
            })
        } else {
            let table_crud_service = TableCRUDService::new(self.web3_service);
            Ok(table_crud_service.insert(&table_name, &key_field_value, &entry).await?)
        }
    }

    async fn execute_query(&self, query: &Query) -> Result<JSONValue, PrecompiledServiceError> {
        if let SetExpr::Select(select) = &query.body {
            // 表名解析
            let from: &Vec<TableWithJoins> = &select.from;
            if from.len() > 1 {
                return Err(PrecompiledServiceError::CustomError {
                    message: "Select from multiple tables is not supported yet".to_string(),
                });
            }
            let table_name = match &from[0].relation {
                TableFactor::Table { name, .. } => (&name.0)[0].value.clone(),
                _ => String::from("")
            };
            if table_name.len() == 0 {
                return Err(PrecompiledServiceError::CustomError {
                    message: "Can't parse table name with the invalid select sql statement".to_string(),
                });
            }
            // 表结构获取
            let (key_field, table_fields) = self.fetch_table_fields(&table_name).await?;

            // 查询字段解析
            let projection: &Vec<SelectItem> = &select.projection;
            let mut select_fields: Vec<String> = match projection.len() {
                0 => table_fields.clone(),
                _ => projection.into_iter().map(|item| match item {
                    SelectItem::UnnamedExpr(expr) => self.get_value_from_expr(&expr).unwrap_or(String::from("")),
                    _ => String::from(""),
                }).filter(|v| v.len() > 0).collect(),
            };
            if select_fields.len() == 0 {
                select_fields = table_fields.clone();
            } else {
                let _ = self.validate_fields(&table_name, &table_fields, &select_fields)?;
            }

            // where 条件解析
            let mut condition = Condition::default();
            let key_field_value = self.parse_selection(&table_name, &key_field, &table_fields, &select.selection, &mut condition)?;

            // limit offset 条件解析
            let limit = match &query.limit {
                Some(limit) => self.get_value_from_expr(limit)
                    .map(|v| v.parse::<i32>().unwrap_or(-1))
                    .unwrap_or(-1),
                None => -1,
            };
            let offset = match &query.offset {
                Some(offset) => self.get_value_from_expr(&offset.value)
                    .map(|v| v.parse::<i32>().unwrap_or(-1))
                    .unwrap_or(-1),
                None => -1,
            };
            if limit > -1 && offset > -1 {
                condition.set_limit(limit as u32, offset as u32);
            }

            // 数据获取
            let table_crud_service = TableCRUDService::new(self.web3_service);
            let records = table_crud_service.select(&table_name, &key_field_value, &condition).await?;
            // 对数据进行解析，只返回指定字段的数据
            if select_fields.len() == table_fields.len() {
                Ok(json!(records))
            } else {
                Ok(json!(
                    records.into_iter().map(|record| {
                        let mut result = json!({});
                        for select_field in &select_fields  {
                            result[select_field.to_owned()] = record.get(select_field).unwrap_or(&json!(null)).clone();
                        }
                        result
                    }).collect::<Vec<JSONValue>>()
                ))
            }
        } else {
            Err(PrecompiledServiceError::CustomError {
                message: "Invalid select sql statement".to_string(),
            })
        }
    }

    async fn execute_delete(&self, name: &ObjectName, selection: &Option<Expr>) -> Result<i32, PrecompiledServiceError> {
        let table_name: String = (&name.0)[0].value.clone();
        let (key_field, table_fields) = self.fetch_table_fields(&table_name).await?;

        let mut condition = Condition::default();
        let key_field_value = self.parse_selection(&table_name, &key_field, &table_fields, &selection, &mut condition)?;

        let table_crud_service = TableCRUDService::new(self.web3_service);
        Ok(table_crud_service.remove(&table_name, &key_field_value, &condition).await?)
    }

    async fn execute_update(&self, name: &ObjectName, assignments: &Vec<Assignment>, selection: &Option<Expr>) -> Result<i32, PrecompiledServiceError> {
        let table_name: String = (&name.0)[0].value.clone();
        let (key_field, table_fields) = self.fetch_table_fields(&table_name).await?;

        let mut entry: HashMap<String, String> = HashMap::new();
        for assignment in assignments.into_iter() {
            let key = assignment.id.value.clone();
            if key_field.eq(&key) {
                return Err(PrecompiledServiceError::CustomError {
                    message: format!("Key field {:?} can't be updated", key),
                });
            }
            if !table_fields.contains(&key) {
                return Err(PrecompiledServiceError::CustomError {
                    message: format!("There is no field {:?} in table {:?}", key, table_name),
                });
            }
            match self.get_value_from_expr(&assignment.value) {
                Some(value) => {
                    entry.insert(key, value);
                },
                None => {},
            };
        }

        let mut condition = Condition::default();
        let key_field_value = self.parse_selection(&table_name, &key_field, &table_fields, &selection, &mut condition)?;

        let table_crud_service = TableCRUDService::new(self.web3_service);
        Ok(table_crud_service.update(&table_name, &key_field_value, &entry, &condition).await?)
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
            Statement::Query(query) =>  self.execute_query(query).await,
            Statement::Delete { table_name, selection } => {
                Ok(json!(self.execute_delete(table_name, selection).await?))
            },
            Statement::Update { table_name, assignments, selection} => {
                Ok(json!(self.execute_update(table_name, assignments, selection).await?))
            },
            Statement::ShowColumns { table_name,.. } => {
                let table_name: String = (&table_name.0)[0].value.clone();
                let table_crud_service = TableCRUDService::new(self.web3_service);
                Ok(json!(table_crud_service.desc(&table_name).await?))
            },
            _ => Err(PrecompiledServiceError::CustomError {
                message: format!("Invalid sql:{:?}", sql),
            }),
        }
    }
}