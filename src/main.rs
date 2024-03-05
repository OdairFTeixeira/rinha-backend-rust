#[macro_use]
extern crate diesel;

use std::{env, net::SocketAddr};

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;

mod handler;
mod model;
mod repository;
mod schema;
mod service;
mod dto;

use handler::client_handler::{
    handle_transaction,
    handle_extract,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();
    let main_port: u16 = env::var("PORT")
        .expect("Port não definida no arquivo .env")
        .parse()
        .expect("Falha ao analisar o valor da porta como um número inteiro");
    let socket_addr = 
        SocketAddr::new("0.0.0.0".parse().expect(""), main_port);

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        
        App::new()
            .wrap(logger)
            .service(handle_transaction)
            .service(handle_extract)
    })
    .bind(socket_addr)?  // Definindo o endereço e a porta do servidor
    .run();

    println!("🔥 Servidor rodando na porta {:?} ✅", main_port);
    server.await
}