use actix_web::{web, App, HttpServer, HttpResponse};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use crate::health_check;

    #[actix_web::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        // This requires changing the return type of `health_check`
        // from `impl Responder` to `HttpResponse` to compile
        // You also need to import it with the `use actix_web::HttpResponse`
        assert_eq!(response.status(), StatusCode::OK)
    }
}