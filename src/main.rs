// Lint rule
#![warn(clippy::all, clippy::pedantic)]
mod editor;
use log4rs;
use editor::Editor;
use log::info;


fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Stating application");
    Editor::default().run();
}
