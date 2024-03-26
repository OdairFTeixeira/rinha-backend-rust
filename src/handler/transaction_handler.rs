use actix::Handler;
use diesel::{QueryResult, RunQueryDsl};
use diesel::{ExpressionMethods, QueryDsl};
use crate::messages::transaction_message::CreateTransaction;
use crate::schema::transaction_schema::transactions::{created_at, value};
use crate::{database::db::DbActor, messages::{transaction_message::FetchTransactionsClient}, model::{client_model::Client, transaction_model::TransactionDatabase}, schema::{client_schema::clients, transaction_schema::transactions::{self, client_id}}};

impl Handler<FetchTransactionsClient> for DbActor {
    type Result = QueryResult<Vec<TransactionDatabase>>;
    
    fn handle(&mut self, msg: FetchTransactionsClient, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch transactions client: unable to stabilish connection");

        transactions::table
        .filter(transactions::client_id.eq(msg.client_id))
        .order(transactions::created_at.desc())
        .load::<TransactionDatabase>(&mut conn)
    }
}

impl Handler<CreateTransaction> for DbActor {
    type Result = QueryResult<usize>;
 
    fn handle(&mut self, msg: CreateTransaction, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch transactions client: unable to stabilish connection");

        diesel::insert_into(transactions::table).values(msg.transaction).execute(&mut conn)
    }
}