use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web, HttpRequest};
use crate::types::config::{Config, Route};
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

    // Match Route based on upstream_path_template
    let matched_route = match_route(&routes, &req);

    let request_path = req.path();
    let resp = reverse_proxy::request_downstream(&matched_route, &request_path).await.unwrap();

    HttpResponse::Ok().body(resp.text().await.unwrap())
}


fn match_route(routes: &Vec<Route>, req: &HttpRequest) -> Route {
    
    for route in routes {
        let hast_placeholder = has_route_placeholder(&route);

        // if route hasnt placeholder, match route based on upstream_path_template
        if !hast_placeholder {
            if route.upstream_path_template == req.path() {
                return route.clone();
            }
        }
    }
    
    return routes[0].clone();
}

fn has_route_placeholder(route: &Route) -> bool {
    let has_placeholder = route.upstream_path_template.contains("{") && route.upstream_path_template.contains("}");
    return has_placeholder;
}