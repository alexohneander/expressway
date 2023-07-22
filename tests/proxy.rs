#[cfg(test)]
mod tests {
    use actix_web::{test, App, web};
    use expressway::handlers::proxy::hello;

    #[actix_web::test]
    async fn test_hello() {
        let mut app = test::init_service(App::new().route("/", web::get().to(hello))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hey there!");
    }
}