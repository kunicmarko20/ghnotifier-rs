#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

fn main() {
    cli::application::Application::run();
}

mod asset;
mod cli;
mod indicator;
mod logger;
mod menu;
mod notifier;
mod github_client;
mod config;
mod settings_window;
mod worker;
