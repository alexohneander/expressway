use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

use crate::configuration::configuration_loader::ConfigurationLoader;

mod configuration;
mod types;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let configuration_loader = ConfigurationLoader {};
    let config= configuration_loader.load("config.json").unwrap();
    println!("Configuration: {:?}", config.global_configuration.base_url);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}