use log::{info, LevelFilter};
use server::{App, AppBuilder};

#[tokio::main]
async fn main() {
  log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

  info!("Starting Application Entrypoint");

    let app = AppBuilder::default().build();
    let backend = async move { app.start().await;};

    tokio::join!(backend);
}
