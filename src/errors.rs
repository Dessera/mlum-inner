use std::fmt;

use actix_web::{error, error::Error as ActixError, http::StatusCode, HttpResponse};
use mongodb::error::Error as MongoError;
use serde::Serialize;

/**
 * All errors that can occur in the application will be wrapped in this struct.
 */
#[derive(Debug, Serialize)]
pub struct WebError {
    pub code: WebErrorStatus,
    pub message: WebErrorMessages,
}

impl WebError {
    pub fn new(code: StatusCode, message: String) -> Self {
        WebError {
            code: WebErrorStatus(code),
            message: WebErrorMessages::from_string(message),
        }
    }
}

// Message for the error response
#[derive(Debug, Serialize, Clone)]
pub struct WebErrorMessages {
    pub error_message: String,
}
impl WebErrorMessages {
    pub fn from_string(message: String) -> Self {
        WebErrorMessages {
            error_message: message,
        }
    }
}

// Status code for the error response
#[derive(Debug)]
pub struct WebErrorStatus(StatusCode);

impl Serialize for WebErrorStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.0.as_u16())
    }
}

// impls

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        self.code.0
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(self.message.clone())
    }
}

impl From<bson::de::Error> for WebError {
    fn from(value: bson::de::Error) -> Self {
        use bson::de::Error::*;
        match value {
            Io(err) => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!("BSON IO error: {}", err)),
            },
            InvalidUtf8String(err) => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!(
                    "BSON Invalid UTF8 string error: {}",
                    err
                )),
            },
            UnrecognizedDocumentElementType {
                key, element_type, ..
            } => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!(
                    "BSON Unrecognized document element type error: {}-{}",
                    element_type, key
                )),
            },
            EndOfStream => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!("BSON End of stream error")),
            },
            DeserializationError { message, .. } => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!(
                    "BSON Deserialization error: {}",
                    message
                )),
            },
            _ => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!("BSON error: {}", value)),
            },
        }
    }
}

impl From<ActixError> for WebError {
    fn from(value: ActixError) -> Self {
        WebError {
            code: WebErrorStatus(value.error_response().status()),
            message: WebErrorMessages::from_string(format!("Actix error: {}", value)),
        }
    }
}

impl From<MongoError> for WebError {
    fn from(value: MongoError) -> Self {
        use mongodb::error::ErrorKind::*;
        let error_kind = value.kind.as_ref();
        match error_kind {
            InvalidArgument { message, .. } => WebError {
                code: WebErrorStatus(StatusCode::BAD_REQUEST),
                message: WebErrorMessages::from_string(format!(
                    "MongoDB Invalid argument error: {}",
                    message
                )),
            },
            Authentication { message, .. } => WebError {
                code: WebErrorStatus(StatusCode::UNAUTHORIZED),
                message: WebErrorMessages::from_string(format!(
                    "MongoDB Authentication error: {}",
                    message
                )),
            },
            DnsResolve { message, .. }
            | Internal { message, .. }
            | ServerSelection { message, .. }
            | InvalidTlsConfig { message, .. }
            | Transaction { message, .. }
            | IncompatibleServer { message, .. }
            | InvalidResponse { message, .. }
            | ConnectionPoolCleared { message, .. } => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!("MongoDB error: {}", message)),
            },
            _ => WebError {
                code: WebErrorStatus(StatusCode::INTERNAL_SERVER_ERROR),
                message: WebErrorMessages::from_string(format!("MongoDB error: {}", value)),
            },
        }
    }
}
