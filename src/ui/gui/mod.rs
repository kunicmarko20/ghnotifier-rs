use arc_guard::ArcGuard;
use crate::infrastructure::factory::notifier_factory::NotifierFactory;
use crate::infrastructure::reqwest::github::github_client::GithubClient;
use crate::infrastructure::reqwest::github::notification::notification_client::NotificationClient;
use crate::infrastructure::factory::logger_factory::LoggerFactory;
use crate::infrastructure::file::config::Config;
use crate::ui::gui::indicator::Indicator;

pub fn init() {
    gtk::init().expect("Unable to initialise gtk.");

    let config = ArcGuard::new(Config::new());
    let indicator = ArcGuard::new(Indicator::new(config.clone()));

    let notification_client = NotificationClient::new(
        GithubClient::new(
        config.clone()
        )
    );

    let mut worker = worker::Worker::new(
        notification_client,
        config.clone(),
        indicator,
        NotifierFactory::new(),
        LoggerFactory::create()
    );

    let config = config.clone();
    std::thread::spawn(move || {
        loop {
            &worker.execute();
            let refresh_time = config.execute(|config| -> u64 {
                let config = config.lock().expect("Unable to lock config.");
                config.get_u64("refresh_time")
            });

            std::thread::sleep(
                std::time::Duration::from_secs(
                    refresh_time
                )
            );
        }
    });

    gtk::main();
}

mod menu;
mod indicator;
pub mod notifier;
pub mod window;
mod worker;
