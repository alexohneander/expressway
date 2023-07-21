use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use log::info;

use crate::configuration::configuration_loader::ConfigurationLoader;

mod configuration;
mod types;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let configuration_loader = ConfigurationLoader {};
    let config= configuration_loader.load("config.json").unwrap();

    info!("Starting server at: {}", &config.global_configuration.base_url);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(manual_hello))
    })
    .bind(config.global_configuration.base_url)?
    .run()
    .await
}