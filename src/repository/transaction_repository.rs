use std::env;

use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel::expression_methods::ExpressionMethods;

use crate::model::transaction_model::{NewTransaction, TransactionDatabase};
use crate::schema::transaction_schema::transactions::{self, created_at};

pub struct TransactionRepository {
    connection: PgConnection
}

impl TransactionRepository {

    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida no arquivo .env");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Self { connection }
    }

    pub fn insert_transaction(&self, new_transaction: &NewTransaction) {
        diesel::insert_into(transactions::table).values(new_transaction).execute(&self.connection).expect("Erro ao realizar transação");
    }

    pub fn get_transaction_by_client_id(&self, id_client: i32) -> Vec<TransactionDatabase> {
        transactions::table
                .filter(transactions::client_id.eq(id_client))
                .order(created_at.desc())
                .load::<TransactionDatabase>(&self.connection)
                .expect("Erro ao buscar transações")
    }
}