#[macro_use] extern crate serde_derive;

mod indicator;
mod menu;
mod notifier;
mod github_client;
mod config;
mod settings;

fn main() {
    gtk::init().unwrap();
    let mut indicator = indicator::Indicator::new(menu::Menu::new());
    let mut notifier = notifier::Notifier::new();
    let client = github_client::GithubClient::new();
    let config = config::Config::new();

    std::thread::spawn(move || {
        loop {
            match client.get_notifications() {
                Ok(notifications) => {
                    indicator.update_label(notifications.len().to_string().as_str());
                    notifier.execute(notifications)
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
