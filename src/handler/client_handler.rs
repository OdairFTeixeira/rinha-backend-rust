use actix::Handler;
use actix_web::HttpResponse;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, QueryResult, RunQueryDsl};
use diesel::ExpressionMethods;
use log::error;
use std::error::Error;

use crate::messages::client_message::SaveClient;
use crate::{database::db::DbActor, messages::client_message::{FetchClient, FetchClientBy}, model::client_model::Client, schema::client_schema::clients};

impl Handler<FetchClient> for DbActor {
    type Result = QueryResult<Vec<Client>>;
    
    fn handle(&mut self, _: FetchClient, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch client: unable to stabilish connection");

        clients::table.get_results::<Client>(&mut conn)
    }
    
}

impl Handler<FetchClientBy> for DbActor {
    type Result = QueryResult<Client>;
    
    fn handle(&mut self, msg: FetchClientBy, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch client by: unable to stabilish connection");

        clients::table.filter(clients::id.eq(msg.client_id)).get_result(&mut conn)
    }
}

impl Handler<SaveClient> for DbActor {
    type Result = QueryResult<Result<Client, diesel::result::Error>>;
    
    fn handle(&mut self, msg: SaveClient, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Fetch client by: unable to establish connection");

        let client = conn.transaction::<Client, diesel::result::Error, _>(|| {
            let _lock_result = diesel::sql_query("SELECT id FROM clients WHERE id = $1 FOR UPDATE")
                .bind::<diesel::sql_types::Int4, _>(msg.client_id)
                .execute(&conn)?;

            let mut client = clients::table.find(msg.client_id).first::<Client>(&conn)?;
            match client.handle_transaction(&msg.new_transaction) {
                Ok(()) => {
                    diesel::update(clients::table.find(msg.client_id))
                    .set(clients::balance.eq(client.balance))
                    .execute(&conn)?;

                    Ok(client)
                },
                Err(_err) => {
                    error!("Erro ao efetuar cliente");
                    Err(diesel::result::Error::NotFound)
                }
            }
        });

        match client {
            Ok(client_ok) => Ok(Ok(client_ok)),
            Err(_err) => {
                error!("Erro ao retornar");
                return Err(diesel::result::Error::NotFound);
            }
        }
    }
}
