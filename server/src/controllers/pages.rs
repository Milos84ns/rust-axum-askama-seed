use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use crate::ui::HomePage;

pub fn create_page_routes() -> Router {
    let mut router = Router::new()
        .route("/",get(handle_homepage));
    router
}

async fn handle_homepage() -> impl IntoResponse {
    let template = HomePage{};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}