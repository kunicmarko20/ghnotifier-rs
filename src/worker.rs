use super::config;
use super::github_client;
use super::indicator;
use std::mem;
use arc_guard::ArcGuard;
use crate::notifier::NotifierFactory;

pub struct Worker {
    client: github_client::GithubClient,
    config: ArcGuard<config::Config>,
    indicator: ArcGuard<indicator::Indicator>,
    notifier_factory: NotifierFactory,
    notified_ids: Vec<String>,
}

unsafe impl Send for Worker{}

impl Worker {
    pub const MAX_NOTIFICATIONS: usize = 10;

    pub fn new(
        client: github_client::GithubClient,
        config: ArcGuard<config::Config>,
        indicator: ArcGuard<indicator::Indicator>,
        notifier_factory: NotifierFactory,
    ) -> Self {
        Worker{
            client,
            config,
            indicator,
            notifier_factory,
            notified_ids: Vec::new()
        }
    }

    pub fn execute(&mut self) {
        let quiet_mode =  self.config.execute(|config| -> String {
            let config = config.lock().unwrap();
            config.get("quiet_mode").unwrap()
        });

        let notifier = self.notifier_factory.from_arg(quiet_mode == "1");

        match &self.client.get_notifications() {
            Ok(notifications) => {
                let number_of_notifications = notifications.len();

                self.indicator.execute(move |indicator| {
                    let mut indicator = indicator.lock().expect("Unable to lock indicator from worker.");
                    indicator.change_notification_number(number_of_notifications.to_string().as_str());
                });

                for notification in notifications.iter().take(Self::MAX_NOTIFICATIONS) {
                    if self.notified_ids.contains(notification.id()) {
                        continue;
                    }

                    notifier.notify(notification);
                }

                mem::swap(&mut self.notified_ids,
                          &mut notifications.iter()
                              .map(|notification| notification.id().to_owned())
                              .collect::<Vec<String>>()
                );
            },
            Err(error) => notifier.error(error.as_str())
        }
    }
}
