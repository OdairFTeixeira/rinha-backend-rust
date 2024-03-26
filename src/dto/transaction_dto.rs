use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TransactionDto {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: NaiveDateTime
}