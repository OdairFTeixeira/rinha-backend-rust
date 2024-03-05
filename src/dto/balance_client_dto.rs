use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::client_model::Client;

#[derive(Deserialize, Serialize)]
pub struct BalanceDto {
    pub saldo: i32,
    pub limite: i32,
}

#[derive(Deserialize, Serialize)]
pub struct BalanceExtractDto {
    pub saldo: i32,
    pub limite: i32,
    pub data_extrato: NaiveDateTime,
}

impl BalanceDto {
    pub fn new(client: Client) -> Self {
        BalanceDto {
            saldo: client.balance,
            limite: client.balance_limit
        }
    }
}

impl BalanceExtractDto {
    pub fn new(client: Client) -> Self {
        BalanceExtractDto {
            saldo: client.balance,
            limite: client.balance_limit,
            data_extrato: Utc::now().naive_utc()
        }
    }
}