use super::notifier;
use super::config;
use super::github_client;
use super::indicator;
use std::mem;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Worker<'a> {
    client: github_client::GithubClient,
    config: &'a config::Config,
    indicator: MutexGuard<'a, indicator::Indicator>,
    notified_ids: Vec<String>
}

impl<'a> Worker<'a> {
    pub const MAX_NOTIFICATIONS: usize = 10;

    fn new(
        client: github_client::GithubClient,
        config: &'a config::Config,
        indicator: MutexGuard<'a, indicator::Indicator>
    ) -> Self {
        Worker{
            client,
            config,
            indicator,
            notified_ids: Vec::new()
        }
    }

    pub fn run(indicator: Arc<Mutex<indicator::Indicator>>) {
        std::thread::spawn(move || {
            let config = config::Config::new();
            let mut worker = Worker::new(
                github_client::GithubClient::new(),
                &config,
                indicator.lock().unwrap()
            );

            loop {
                &worker.execute();

                std::thread::sleep(
                    std::time::Duration::from_secs(
                        config.get("refresh_time").unwrap().parse::<u64>().unwrap()
                    )
                );
            }
        });
    }

    fn execute(&mut self) {
        match &self.client.get_notifications() {
            Ok(notifications) => {
                &self.indicator.update_label(notifications.len().to_string().as_str());

                if &self.config.get("quiet_mode").unwrap() == "1" {
                    return;
                }

                for notification in notifications.iter().take(Self::MAX_NOTIFICATIONS) {
                    if self.notified_ids.contains(notification.id()) {
                        continue;
                    }

                    notifier::Notifier::success(notification.title(), notification.body());
                }

                mem::swap(&mut self.notified_ids,
                          &mut notifications.iter()
                              .map(|notification| notification.id().to_owned())
                              .collect::<Vec<String>>()
                );
            },
            Err(error) => notifier::Notifier::error(error.as_str())
        }
    }
}
