use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::transaction_model::NewTransaction;
use crate::schema::client_schema::clients;
use std::error::Error;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = clients)]
pub struct Client {
    pub id: i32,
    pub balance_limit: i32,
    pub balance: i32,
}

impl Client {
    pub fn handle_transaction(&mut self, transaction: &NewTransaction) -> Result<(), Box<dyn Error>> {
        if transaction.transaction_type == "c" {
            self.balance += transaction.value;
        } else if transaction.transaction_type == "d" {
            if (self.balance - transaction.value).abs() > self.balance_limit {
                self.balance -= transaction.value;
            } else {
                return Err("Limite insuficiente para a transação".into());
            }
        } else {
            return Err("Tipo de transação inválido".into());
        }
        Ok(())
    }
}