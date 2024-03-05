use actix_web::HttpResponse;

use crate::dto::balance_client_dto::BalanceDto;
use crate::dto::client_handler_dto::SubmitTransactionRequest;
use crate::dto::extract_client_dto::ExtractClientDto;
use crate::model::transaction_model::NewTransaction;
use crate::repository::client_repository::ClientRepository;
use crate::repository::transaction_repository::TransactionRepository;

pub struct TransactionService {
    client_repository: ClientRepository,
    transaction_repository: TransactionRepository
}

impl TransactionService {
    pub fn new() -> Self {
        let client_repository = ClientRepository::new();
        let transaction_repository = TransactionRepository::new();
        Self { client_repository, transaction_repository }
    }

    pub fn add_transaction(&self, id_client: i32, submit_transaction: SubmitTransactionRequest) -> Result<BalanceDto, HttpResponse> {
        match self.client_repository.find_user(id_client) {
            Ok(mut client) => {
                let new_transaction = NewTransaction::new(id_client, submit_transaction);
                if let Err(error) = client.handle_transaction(&new_transaction) {
                    return Err(HttpResponse::UnprocessableEntity().body(error.to_string()));
                }
                self.transaction_repository.insert_transaction(&new_transaction);
                self.client_repository.update_amount(id_client, client.balance);
                return Ok(BalanceDto::new(client));
            },
            Err(response) => Err(response),
        }
    }

    pub fn get_extract(&self, id_client: i32) -> Result<ExtractClientDto, HttpResponse> {
        match self.client_repository.find_user(id_client) {
            Ok(client) => {
                let transactions = self.transaction_repository.get_transaction_by_client_id(id_client);
                Ok(ExtractClientDto::new(client, transactions))
            },
            Err(response) => Err(response),
        }
    }
}