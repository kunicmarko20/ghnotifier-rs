use std::mem;
use arc_guard::ArcGuard;
use crate::infrastructure::factory::notifier_factory::NotifierFactory;
use crate::infrastructure::logger::Logger;
use crate::infrastructure::reqwest::github::notification::notification_client::NotificationClient;
use crate::infrastructure::file::config::Config;
use crate::ui::gui::indicator::Indicator;

pub struct Worker {
    notification_client: NotificationClient,
    config: ArcGuard<Config>,
    indicator: ArcGuard<Indicator>,
    notifier_factory: NotifierFactory,
    logger: Box<Logger>,
    notified_ids: Vec<String>,
}

unsafe impl Send for Worker{}

impl Worker {
    pub const MAX_NOTIFICATIONS: usize = 10;

    pub fn new(
        notification_client: NotificationClient,
        config: ArcGuard<Config>,
        indicator: ArcGuard<Indicator>,
        notifier_factory: NotifierFactory,
        logger: Box<Logger>
    ) -> Self {
        Worker{
            notification_client,
            config,
            indicator,
            notifier_factory,
            logger,
            notified_ids: Vec::new()
        }
    }

    pub fn execute(&mut self) {
        let quiet_mode =  self.config.execute(|config| -> bool {
            let config = config.lock().expect("Unable to lock config.");
            config.get_bool("quiet_mode")
        });

        let notifier = self.notifier_factory.from_arg(quiet_mode);

        let notifications = match self.notification_client.notifications() {
            Ok(notifications) => notifications,
            Err(message) => {
                self.logger.log(&message);
                return;
            }
        };

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
    }
}
