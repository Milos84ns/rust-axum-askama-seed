use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use server::{App, AppBuilder};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

  info!("Starting Application Entrypoint");

    let app = AppBuilder::build();
    app.start().await;
}
