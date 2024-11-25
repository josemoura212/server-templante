use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebhookPayload {
    // Defina os campos do payload do webhook aqui
    event: String,
    data: serde_json::Value,
}

pub async fn webhook_handler(payload: web::Json<WebhookPayload>) -> impl Responder {
    info!("Recebido webhook: {:?}", payload);
    HttpResponse::Ok().json(serde_json::json!({"status": "success"}))
}
