use axum::extract::State;
use axum::handler::Handler;
use axum::Router;
use crate::controllers::api::create_api_routes;
use crate::controllers::pages::create_page_routes;
use crate::PORT;
use log::{info, LevelFilter};
use crate::AppStatus::NOT_STARTED;

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn start(self){
        let address = format!("0.0.0.0:{}", PORT);
        let listener = tokio::net::TcpListener::bind(address.clone())
            .await
            .unwrap();
        info!("Server started at http://{}", address);
        axum::serve(listener, self.router).await.unwrap();
        }
    }

pub struct AppBuilder {
    port: u16,
    app_state: AppState,
}

impl Default for AppBuilder {
    fn default() -> Self {
        AppBuilder{ port:0, app_state: AppState{status:NOT_STARTED}}
    }
}

impl AppBuilder {
    pub fn with_state(self, app_state: AppState) -> Self {
        Self { app_state, ..self }
    }
    pub fn build(self)-> App {
        let main_router = Router::new()
            .with_state(self.app_state)
            .merge(create_page_routes())
            .merge(create_api_routes());
        App {
            router: main_router,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub status:AppStatus
}

#[derive(Clone)]
pub enum AppStatus {
    NOT_STARTED,
    STARTED,
    RUNNING,
    ERROR
}
