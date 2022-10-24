use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponseBody {
    message: String,
    code: u16,
}

#[derive(Debug, Display, Error)]
pub struct DatabaseError {
    pub error_type: DieselError,
}

impl From<DieselError> for DatabaseError {
    fn from(err: diesel::result::Error) -> DatabaseError {
        DatabaseError { error_type: err }
    }
}

impl error::ResponseError for DatabaseError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            diesel::result::Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self.error_type {
            diesel::result::Error::NotFound => String::from("Item not found"),
            _ => String::from("unknown error"),
        };
        let response_body = ErrorResponseBody {
            message,
            code: self.status_code().as_u16(),
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }
}
