#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

fn main() {
    ui::cli::application::Application::run();
}

pub mod ui;
mod infrastructure;
