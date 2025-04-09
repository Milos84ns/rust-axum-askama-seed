use std::io::ErrorKind;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

enum AppError {
    JsonError,
    UiError,
    AppFileNotFound,
    AppErrUnknown
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
          ErrorKind::NotFound => AppError::AppFileNotFound,
            _ => AppError::AppErrUnknown
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    pub status_code: StatusCode,
    pub error: String,
}

impl From<AppError> for ErrorPage {
    fn from(err: AppError) -> Self {
        match err {
            AppError::UiError => ErrorPage { status_code: StatusCode::INTERNAL_SERVER_ERROR, error: err.to_string() },
            AppError::JsonError => ErrorPage { status_code: StatusCode::BAD_REQUEST, error: err.to_string() },
            AppError::AppFileNotFound => ErrorPage { status_code: StatusCode::NOT_FOUND, error: err.to_string() },
            AppError::AppErrUnknown => ErrorPage { status_code: StatusCode::INTERNAL_SERVER_ERROR, error: err.to_string() }
        }
    }
}

// impl from to custom error needed
// Part 1 from std::io:: error to Error Page
impl From<std::io::Error> for ErrorPage {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
           ErrorKind::NotFound => ErrorPage { status_code: StatusCode::NOT_FOUND, error: err.to_string() },
            _ => ErrorPage { status_code: StatusCode::INTERNAL_SERVER_ERROR, error: err.to_string() }
        }
    }
}

// impl response for custom error
// Part 2 Error page IntoResponse
// now std::fs::read_dir(path)? can be called and error propagated to ErrorPage
impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        (self.status_code, self.error).into_response()
    }
}