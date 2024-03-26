
use actix::{dev::Request, Addr};
use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use log::info;

use crate::{database::db::{AppState, DbActor}, dto::{balance_client_dto::BalanceDto, client_handler_dto::{ClientIdentifier, SubmitTransactionRequest}, extract_client_dto::ExtractClientDto}, messages::{client_message::{FetchClient, FetchClientBy, SaveClient}, transaction_message::{CreateTransaction, FetchTransactionsClient}}, model::{client_model::Client, transaction_model::{NewTransaction, TransactionDatabase}}};

#[get("/clientes/{id}/extrato")]
pub async fn handle_extract(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    
    let client_result: Result<Client, HttpResponse> = match db.send(FetchClientBy { client_id: id.clone() }).await {
        Ok(Ok(info)) => Ok(info),
        Ok(Err(_)) => Err(HttpResponse::NotFound().json("No users found")),
        _ => Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
    };

    if client_result.is_err() {
        return HttpResponse::NotFound().json("Erro ao processar a solicitação");
    }

    let transactions_result: Result<Vec<TransactionDatabase>, HttpResponse>  = match db.send(FetchTransactionsClient { client_id: id.clone() }).await {
        Ok(Ok(info)) => Ok(info),
        Ok(Err(_)) => Err(HttpResponse::NotFound().json("No users found")),
        _ => Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
    };

    if transactions_result.is_err() {
        return HttpResponse::InternalServerError().json("Erro ao processar a solicitação");
    }

    match (client_result, transactions_result) {
        (Ok(client), Ok(transactions)) => {
            HttpResponse::Ok().json(ExtractClientDto::new(client, transactions))
        },
        _ => HttpResponse::InternalServerError().json("Erro ao processar a solicitação"),
    }
}

#[post("/clientes/{id}/transacoes")]
pub async fn handle_transaction(state: Data<AppState>, id: web::Path<i32>, resquest: web::Json<SubmitTransactionRequest>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    info!("Efetuando {:?} de {:?} para o usuario {:?}", resquest.tipo, resquest.valor, id.clone());

    if resquest.descricao.len() > 10 || resquest.descricao.len() == 0 || resquest.valor < 0 {
        return HttpResponse::UnprocessableEntity().body("Description too long");
    }

    let new_transaction = NewTransaction::new(id.clone(), resquest.into_inner());


    let error = db.send(SaveClient { client_id: id.clone(), new_transaction: new_transaction.clone() }).await;
    if error.unwrap().is_err() {
        return HttpResponse::UnprocessableEntity().body("Verifique o limite de sua conta");
    }

    let client_result: Result<Client, HttpResponse> = match db.send(FetchClientBy { client_id: id.clone() }).await {
        Ok(Ok(info)) => Ok(info),
        Ok(Err(_)) => Err(HttpResponse::NotFound().json("No users found")),
        _ => Err(HttpResponse::InternalServerError().json("Unable to retrieve users")),
    };

    let error_transaction = db.send(CreateTransaction { transaction: new_transaction.clone() }).await;
    if error_transaction.is_err() {
        return HttpResponse::InternalServerError().body("body");
    }

    HttpResponse::Ok().json(BalanceDto::new(client_result.unwrap()))
}

#[get("/clientes")]
pub async fn find_all(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchClient).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}