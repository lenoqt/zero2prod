use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use unicode_segmentation::UnicodeSegmentation;

use crate::authentication::AuthError;

pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub async fn check_password_length(password: &str) -> Result<(), AuthError> {
    return match password.graphemes(true).count() {
        count if count <= 11 => Err(AuthError::InvalidCredentials(anyhow::anyhow!(
            "Password length is too short."
        ))),
        count if count >= 129 => Err(AuthError::InvalidCredentials(anyhow::anyhow!(
            "The password should not contain more than 128 characters."
        ))),
        _ => Ok(()),
    };
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserver for logging purposes.
pub fn e400<T: std::fmt::Debug + std::fmt::Display>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}
