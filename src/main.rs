#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

fn main() {
    gtk::init().unwrap();

    let config = Arc::new(
        Mutex::new(
            config::Config::new()
        )
    );

    let menu = menu::Menu::new(config.clone());

    let indicator = Arc::new(
        Mutex::new(
            indicator::Indicator::new(menu)
        )
    );

    let github_client = github_client::GithubClient::new(
        config.clone()
    );

    let mut worker = worker::Worker::new(
        github_client,
        config.clone(),
        indicator.clone()
    );

    let config = config.clone();
    std::thread::spawn(move || {
        loop {
            &worker.execute();
            let config_thread = config.clone();
            let config_thread = config_thread.lock().unwrap();
            let refresh_time = config_thread.get("refresh_time").unwrap().parse::<u64>().unwrap();
            drop(config_thread);
            std::thread::sleep(
                std::time::Duration::from_secs(
                    refresh_time
                )
            );
        }
    });

    gtk::main();
}

mod indicator;
mod menu;
mod notifier;
mod github_client;
mod config;
mod settings_window;
mod worker;
