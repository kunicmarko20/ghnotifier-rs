#[macro_use] extern crate serde_derive;

fn main() {
    cli::application::Application::run();
}

mod asset;
mod cli;
mod indicator;
mod menu;
mod notifier;
mod github_client;
mod config;
mod settings_window;
mod worker;
