use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web};
use crate::types::config::Config;
use crate::services::reverse_proxy;

pub struct AppStateWithConfig {
    pub config: Mutex<Config>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn proxy(data: web::Data<AppStateWithConfig>) -> impl Responder {
    // request downstream
    let config = data.config.lock().unwrap();
    let routes = &config.routes;

    reverse_proxy::request_downstream(&routes[0]);

    HttpResponse::Ok().body("Hey there!")
}
