use actix_web::HttpResponse;

use crate::error::sqlx::insert::InsertErr;

use super::into_http_err::IntoHttpErr;

pub trait CreatedReponse {
    fn created_response(self) -> Result<HttpResponse, actix_web::Error>;
}

impl<T: serde::Serialize> CreatedReponse for Result<T, InsertErr> {
    fn created_response(self) -> Result<HttpResponse, actix_web::Error> {
        match self {
            Ok(body) => Ok(HttpResponse::Created().json(body)),
            Err(e) => Err(e.http_err().into()),
        }
    }
}
