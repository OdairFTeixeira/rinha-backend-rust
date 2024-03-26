use actix::Message;
use diesel::QueryResult;
use crate::model::transaction_model::{TransactionDatabase, NewTransaction};


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<TransactionDatabase>>")]
pub struct FetchTransactionsClient {
    pub client_id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct CreateTransaction {
    pub transaction: NewTransaction
}

