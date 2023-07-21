use actix_web::{Responder, HttpResponse};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}