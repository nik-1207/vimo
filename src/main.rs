// Lint rule
#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;
use log::info;
use log4rs::config::Deserializers;


fn main() {
    log4rs::init_file("log4rs.yml", Deserializers::default()).unwrap();
    info!("Stating application");
    Editor::default().run();
}
