use diesel::Queryable;

use crate::dto::transaction_dto::TransactionDto;
use crate::schema::transaction_schema::transactions;
use crate::dto::client_handler_dto::SubmitTransactionRequest;

#[derive(Insertable, Debug, Clone)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub client_id: i32,
    pub value: i32,
    pub transaction_type: String,
    pub description: String,
}

#[derive(Queryable, Debug)]
pub struct TransactionDatabase {
    pub id: i32,
    pub client_id: i32,
    pub value: i32,
    pub transaction_type: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime
}

impl NewTransaction {
    pub fn new(client: i32, transaction_request: SubmitTransactionRequest) -> Self {
        NewTransaction  {
            client_id: client,
            value: transaction_request.valor,
            transaction_type: transaction_request.tipo,
            description: transaction_request.descricao
         }
    }
}

impl TransactionDatabase {
    pub fn to_dto(&self) -> TransactionDto {
        TransactionDto {
            valor: self.value,
            tipo: self.transaction_type.clone(),
            descricao: self.description.clone(),
            realizada_em: self.created_at.clone()
        }
    }
}