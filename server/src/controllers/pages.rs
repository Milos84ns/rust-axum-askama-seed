use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use crate::ui::{ChartsPage, ErrorPage, HomePage};

pub fn create_page_routes() -> Router {
    let mut router = Router::new()
        .route("/",get(handle_homepage))
        .route("/charts",get(handle_charts))
        .route("/error",get(handle_error_page));
    router
}

async fn handle_homepage() -> impl IntoResponse {
    let template = HomePage{};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn handle_charts() -> impl IntoResponse {
    let template = ChartsPage{};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn handle_error_page() -> impl IntoResponse {
    let template = ErrorPage{
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error: "Some error I don't know what".to_string(),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::INTERNAL_SERVER_ERROR, Html(reply_html).into_response())
}