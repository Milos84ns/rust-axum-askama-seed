
use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "homepage.html")]
pub struct HomePage {}


#[derive(Template)]
#[template(path = "chartjs.html")]
pub struct ChartsPage {}


