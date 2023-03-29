use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use mongodb::error::Error as MongoError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum WebError {
    DBError(String),
    BSONError(String),
    ActixError(String),
    NotFound(String),
}
#[derive(Debug, Serialize)]
pub struct WebErrorResponse {
    error_message: String,
}

impl WebError {
    fn error_response(&self) -> String {
        match self {
            WebError::DBError(msg) => {
                println!("Database error occured: {:?}", msg);
                format!("Database error: {:?}", msg)
            }
            WebError::ActixError(msg) => {
                println!("Server error occured: {:?}", msg);
                format!("Server error: {:?}", msg)
            }
            WebError::NotFound(msg) => {
                println!("Not found error occured: {:?}", msg);
                msg.into()
            }
            WebError::BSONError(msg) => {
                println!("BSON error occured: {:?}", msg);
                format!("BSON error: {:?}", msg)
            }
        }
    }
}

impl error::ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::DBError(_) | WebError::ActixError(_) | WebError::BSONError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(WebErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for WebError {
    fn from(err: actix_web::error::Error) -> Self {
        WebError::ActixError(err.to_string())
    }
}

impl From<MongoError> for WebError {
    fn from(err: MongoError) -> Self {
        WebError::DBError(err.to_string())
    }
}

impl From<bson::de::Error> for WebError {
    fn from(err: bson::de::Error) -> Self {
        WebError::BSONError(err.to_string())
    }
}
