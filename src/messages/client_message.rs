use actix::Message;
use diesel::QueryResult;
use crate::model::{client_model::Client, transaction_model::NewTransaction};


#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Client>>")]
pub struct FetchClient;


#[derive(Message)]
#[rtype(result = "QueryResult<Client>")]
pub struct FetchClientBy {
    pub client_id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<Result<Client, diesel::result::Error>>")]
pub struct SaveClient {
    pub client_id: i32,
    pub new_transaction: NewTransaction
}