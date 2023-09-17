//! src/routes/login/get.rs
use actix_web::{http::header::ContentType, web, HttpResponse};
use hmac::{Hmac, Mac};
use htmlescape::encode_minimal;
use secrecy::ExposeSecret;

use crate::startup::HmacSecret;

#[derive(serde::Deserialize)]
pub struct QueryParameters {
    error: String,
    tag: String,
}

impl QueryParameters {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

pub async fn login_form(
    query: Option<web::Query<QueryParameters>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let error_html = match query {
        None => "".into(),
        Some(query) => match query.0.verify(&secret) {
            Ok(error) => {
                format!("<p><i>{}</i></p>", encode_minimal(&error))
            }
            Err(e) => {
                tracing::warn!(
                    error.message = %e,
                    error.cause_chain = ?e,
                    "Failed to verify query parameters using HMAC tag"
                );
                "".into()
            }
        },
    };

    let body = format!(
        r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            <body>
                {}
                <form action="/login" method="post">
                    <label>Username
                        <input
                            type="text"
                            placeholder="Enter Username"
                            name="username"
                            >
                    </label>
                    <label>Password
                        <input
                            type="password"
                            placeholder="Enter Password"
                            name="password"
                        >
                    </label>
                    <button type="submit">Login</button>
                </form>
            </body>
            </html>
            "#,
        error_html
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)
}
