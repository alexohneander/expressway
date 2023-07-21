use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

use crate::configuration::configuration_loader::ConfigurationLoader;
use crate::handlers::proxy;

mod configuration;
mod handlers;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load configuration
    let configuration_loader = ConfigurationLoader {};
    let config = configuration_loader.load("config.json").unwrap();

    info!(
        "Starting server at: {}",
        &config.global_configuration.base_url
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(&config.global_configuration.base_url)?
    .run()
    .await
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    let configuration_loader = ConfigurationLoader {};
    let config = configuration_loader.load("config.json").unwrap();

    let mut scopes = actix_web::Scope::new("");

    for route in config.routes {
        //define your methods here
        for method in route.upstream_http_methods {
            if method == "get" {
                scopes = scopes.route(&route.upstream_path_template, web::get().to(proxy::hello));
            } else if method == "post" {
                scopes = scopes.route(&route.upstream_path_template, web::post().to(proxy::hello));
            }
        }
    }

    //add it to the server
    cfg.service(scopes);
}
