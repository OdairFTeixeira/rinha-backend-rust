use std::env;

use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::model::client_model::Client;
use crate::schema::client_schema::clients::{self, balance};

pub struct ClientRepository {
    connection: PgConnection,
}

impl ClientRepository {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida no arquivo .env");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Self { connection }
    }

    pub fn find_user(&self, id: i32) -> Result<Client, HttpResponse> {
        match clients::table.find(id).first::<Client>(&self.connection) {
            Ok(client) => Ok(client),
            Err(_) => Err(HttpResponse::NotFound().finish())
        }
    }

    pub fn update_amount(&self, client_id: i32, new_amount: i32) {
        let _ = diesel::update(clients::table.find(client_id)).set(balance.eq(new_amount)).execute(&self.connection);
    }
}