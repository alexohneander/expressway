#[cfg(test)]
mod tests {
    use actix_web::{http, test, App, web};
    use expressway::handlers::proxy;

    #[actix_web::test]
    async fn test_get_route() {
        let mut app = test::init_service(
            App::new()
                .route("/", web::get().to(proxy::hello))
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_route() {
        let mut app = test::init_service(
            App::new()
                .route("/", web::post().to(proxy::hello))
        ).await;

        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}