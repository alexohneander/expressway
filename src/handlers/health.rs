use actix_web::{Responder, HttpResponse};

pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}