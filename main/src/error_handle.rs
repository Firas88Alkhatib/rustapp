use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use diesel::result::Error as DieselError;
use reqwest::Error as RequestError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseErrorBody {
    message: String,
    code: u16,
}
#[derive(Debug)]
pub enum RepositoryErrorType {
    NotFound,
    AlreadyExist,
    ConnectionError,
    UnknownError,
}
#[derive(Debug, Display)]
#[display(fmt = "{:?}", error_type)]
pub struct RepositoryError {
    pub error_type: RepositoryErrorType,
}

impl From<DieselError> for RepositoryError {
    fn from(err: DieselError) -> RepositoryError {
        let error_type = match err {
            DieselError::NotFound => RepositoryErrorType::NotFound,
            _ => RepositoryErrorType::UnknownError,
        };
        return RepositoryError { error_type };
    }
}
impl From<RequestError> for RepositoryError {
    fn from(err: RequestError) -> RepositoryError {
        if err.is_status() {
            let error_type = match err.status() {
                Some(status_code) => match status_code {
                    StatusCode::NOT_FOUND => RepositoryErrorType::NotFound,
                    StatusCode::CONFLICT => RepositoryErrorType::AlreadyExist,
                    _ => RepositoryErrorType::UnknownError,
                },
                _ => RepositoryErrorType::UnknownError,
            };
            return RepositoryError { error_type };
        } else {
            return RepositoryError {
                error_type: RepositoryErrorType::ConnectionError,
            };
        }
    }
}

impl ResponseError for RepositoryError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            RepositoryErrorType::NotFound => StatusCode::NOT_FOUND,
            RepositoryErrorType::AlreadyExist => StatusCode::CONFLICT,
            RepositoryErrorType::UnknownError | _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self.error_type {
            RepositoryErrorType::NotFound => String::from("Item not found"),
            RepositoryErrorType::AlreadyExist => String::from("Item already exist"),
            RepositoryErrorType::ConnectionError => String::from("Internal connection error"),
            RepositoryErrorType::UnknownError => String::from("Unknown error"),
        };
        let response_body = ResponseErrorBody {
            message,
            code: self.status_code().as_u16(),
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }
}
