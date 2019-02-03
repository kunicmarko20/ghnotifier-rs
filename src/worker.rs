use super::notifier;
use super::config;
use super::github_client;
use super::indicator;
use std::mem;
use std::sync::{Arc, Mutex};

pub struct Worker {
    client: github_client::GithubClient,
    config: Arc<Mutex<config::Config>>,
    indicator:Arc<Mutex<indicator::Indicator>>,
    notified_ids: Vec<String>
}

unsafe impl Send for Worker{}

impl Worker {
    pub const MAX_NOTIFICATIONS: usize = 10;

    pub fn new(
        client: github_client::GithubClient,
        config: Arc<Mutex<config::Config>>,
        indicator: Arc<Mutex<indicator::Indicator>>,
    ) -> Self {
        Worker{
            client,
            config,
            indicator,
            notified_ids: Vec::new()
        }
    }

    pub fn execute(&mut self) {
        match &self.client.get_notifications() {
            Ok(notifications) => {
                let indicator = &self.indicator.clone();
                let mut indicator = indicator.lock().unwrap();
                indicator.change_notification_number(notifications.len().to_string().as_str());

                let config = &self.config.clone();
                let config = config.lock().unwrap();
                if config.get("quiet_mode").unwrap() == "1" {
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
