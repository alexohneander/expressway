use std::sync::Mutex;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use log::info;

use crate::configuration::configuration_loader;
use crate::handlers::proxy::{self, AppStateWithConfig};
use crate::types::config::Config;

mod configuration;
mod handlers;
mod types;
mod services;

// Initialize global configuration
lazy_static::lazy_static! {
    static ref CONFIG: Config = configuration_loader::load("config.json");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // Put the config in an app data
    let config_data = web::Data::new(AppStateWithConfig {
        config: Mutex::new(CONFIG.clone()),
    });

    info!(
        "Starting server at: {}",
        &CONFIG.global_configuration.base_url
    );

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(config_data.clone())
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(&CONFIG.global_configuration.base_url)?
    .run()
    .await
}

/// ### Configure Routes dynamically with the config
fn configure_routes(cfg: &mut web::ServiceConfig) {
    let mut scopes = actix_web::Scope::new("");

    for route in &CONFIG.routes {
        //define your methods here
        for method in &route.upstream_http_methods {
            if method == "get" {
                scopes = scopes.route(&route.upstream_path_template, web::get().to(proxy::proxy));
            } else if method == "post" {
                scopes = scopes.route(&route.upstream_path_template, web::post().to(proxy::hello));
            }
        }
    }

    //add it to the server
    cfg.service(scopes);
}
