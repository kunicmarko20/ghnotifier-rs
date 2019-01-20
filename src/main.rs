#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

fn main() {
    gtk::init().unwrap();

    let indicator = Arc::new(Mutex::new(
        indicator::Indicator::new(menu::Menu::new())
    ));

    worker::Worker::run(indicator.clone());

    gtk::main();
}

mod indicator;
mod menu;
mod notifier;
mod github_client;
mod config;
mod settings_window;
mod worker;
