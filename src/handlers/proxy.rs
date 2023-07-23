use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web, HttpRequest};
use crate::types::config::Config;
use crate::services::reverse_proxy;

pub struct AppStateWithConfig {
    pub config: Mutex<Config>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn proxy(data: web::Data<AppStateWithConfig>, req: HttpRequest) -> impl Responder {
    // request downstream
    let config = data.config.lock().unwrap();
    let routes = &config.routes;

    // TODO: Match route based on request path
    // Match Route based on request path

    let request_path = req.path();

    let resp = reverse_proxy::request_downstream(&routes[0], &request_path).await.unwrap();

    HttpResponse::Ok().body(resp.text().await.unwrap())
}
