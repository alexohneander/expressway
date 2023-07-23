use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web};
use crate::types::config::Config;

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
    println!("{:?}", routes);

    HttpResponse::Ok().body("Hey there!")
}
