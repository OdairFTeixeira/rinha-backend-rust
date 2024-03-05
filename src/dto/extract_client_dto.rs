use serde::{Deserialize, Serialize};

use crate::model::{client_model::Client, transaction_model::TransactionDatabase};

use super::{balance_client_dto::BalanceExtractDto, transaction_dto::TransactionDto};

#[derive(Deserialize, Serialize)]
pub struct ExtractClientDto {
    pub saldo: BalanceExtractDto,
    pub ultimas_transacoes: Vec<TransactionDto>,
}


impl ExtractClientDto {
    pub fn new(client: Client, transactions: Vec<TransactionDatabase>) -> ExtractClientDto {
        ExtractClientDto {
            saldo: BalanceExtractDto::new(client),
            ultimas_transacoes: transactions.iter().map(|transaction| transaction.to_dto()).collect()
        }
    }
}