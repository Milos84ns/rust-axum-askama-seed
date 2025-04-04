use axum::Router;
use crate::controlers::api::create_api_routes;
use crate::controlers::pages::create_page_routes;
use crate::PORT;
use log::{info, LevelFilter};

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn start(self){
        let address = format!("0.0.0.0:{}", PORT);
        let listener = tokio::net::TcpListener::bind(address.clone())
            .await
            .unwrap();
        info!("Server started at {}", address);
        axum::serve(listener, self.router).await.unwrap();
        }

    }

pub struct AppBuilder {
    port: u16,

}

impl AppBuilder {
    pub fn build()-> App {
        let main_router = Router::new()
            .merge(create_page_routes())
            .merge(create_api_routes());

        App {
            router: main_router,
        }
    }
}