use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ClientIdentifier {
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct SubmitTransactionRequest {
    pub valor: i32,
    pub tipo: String,
    pub descricao : String
}