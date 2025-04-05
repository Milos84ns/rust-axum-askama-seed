mod app;
mod ui;
mod controllers;

pub use app::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed, Clone)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
struct Assets;