mod app;
mod controllers;
mod ui;

pub use app::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed, Clone)]
#[folder = "assets"]
struct Assets;