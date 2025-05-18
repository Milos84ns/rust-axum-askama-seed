
use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomePage {}


#[derive(Template)]
#[template(path = "chartjs.html")]
pub struct ChartsPage {}


#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    pub status_code: StatusCode,
    pub error: String,
}



