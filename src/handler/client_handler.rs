use actix_web::{post, web, HttpResponse, Responder, get};
use crate::{dto::client_handler_dto::{
    ClientIdentifier,
    SubmitTransactionRequest
}, service::transaction_service::TransactionService};



#[post("/clientes/{id}/transacoes")]
pub async fn handle_transaction(id: web::Path<ClientIdentifier>, resquest: web::Json<SubmitTransactionRequest>) -> impl Responder {
    let transaction_service = TransactionService::new();
    match transaction_service.add_transaction(id.into_inner().id, resquest.into_inner()) {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(error_response) => error_response
    }
} 

#[get("/clientes/{id}/extrato")]
pub async fn handle_extract(id: web::Path<ClientIdentifier>) -> impl Responder {
    let transaction_service = TransactionService::new();
    match transaction_service.get_extract(id.into_inner().id) {
        Ok(extract) => HttpResponse::Ok().json(extract),
        Err(error_response) => error_response
    }
}