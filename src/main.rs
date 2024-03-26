#[macro_use]
extern crate diesel;

use database::db::{get_pool, AppState, DbActor};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use std::{env, net::SocketAddr};
use actix::SyncArbiter;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenv::dotenv;

mod database;
mod messages;
mod handler;
mod schema;
mod model;
mod api;
mod dto;

use api::client_api::{
    handle_extract,
    find_all
};

use crate::api::client_api::handle_transaction;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(10, move || DbActor(pool.clone()));

    let main_port: u16 = env::var("PORT")
        .expect("Port não definida no arquivo .env")
        .parse()
        .expect("Falha ao analisar o valor da porta como um número inteiro");
    let socket_addr = 
        SocketAddr::new("0.0.0.0".parse().expect(""), main_port);

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        
        App::new()
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .wrap(logger)
            .service(handle_extract)
            .service(find_all)
            .service(handle_transaction)
    })
    .bind(socket_addr)?  // Definindo o endereço e a porta do servidor
    .run();

    println!("🔥 Servidor rodando na porta {:?} ✅", main_port);
    server.await
}