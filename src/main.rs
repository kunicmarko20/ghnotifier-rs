#[macro_use] extern crate serde_derive;

use std::sync::{Arc, Mutex};

fn main() {
    gtk::init().unwrap();
    let indicator = Arc::new(Mutex::new(
        indicator::Indicator::new(menu::Menu::new())
    ));
    let client = github_client::GithubClient::new();
    let config = config::Config::new();

    std::thread::spawn(move || {
        let mut indicator = indicator.lock().unwrap();
        let mut notified_ids: Vec<String> = Vec::new();

        loop {
            match client.get_notifications() {
                Ok(notifications) => {
                    for notification in notifications.iter().take(notifier::Notifier::MAX_NOTIFICATIONS) {
                        if notified_ids.contains(notification.id()) {
                            continue;
                        }

                        notifier::Notifier::success(notification.title(), notification.body());
                    }

                    notified_ids = notifications.iter()
                        .map(|notification| notification.id().to_owned())
                        .collect::<Vec<String>>();

                    indicator.update_label(notifications.len().to_string().as_str());
                },
                Err(error) => notifier::Notifier::error(error.as_str())
            }

            std::thread::sleep(
                std::time::Duration::from_secs(
                    config.get("refresh_time").unwrap().parse::<u64>().unwrap()
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
