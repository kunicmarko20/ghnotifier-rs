use super::indicator::Indicator;
use super::github_client::GithubClient;
use super::github_client::Notification;

pub struct Notifier {
    indicator: Indicator,
    notified_ids: Vec<String>
}

impl Notifier {
    pub fn new(indicator: Indicator) -> Notifier {
        Notifier{indicator, notified_ids: Vec::new()}
    }

    pub fn execute(&mut self) {
        match GithubClient::new().get_notifications() {
            Ok(notifications) => self.ok(notifications),
            Err(error) => Self::error(error.as_str())
        }
    }

    fn ok(&mut self, notifications: Vec<Notification>) {
        for notification in notifications.iter() {
            if self.notified_ids.contains(notification.id()) {
                continue;
            }

            Self::send(
                notification.title(),
                notification.body(),
                None
            );

            &self.notified_ids.push(notification.id().to_owned());
        }

        &self.indicator.update_label(notifications.len().to_string().as_str());
    }

    fn error(body: &str) {
        Self::send(
            "Something went wrong",
            body,
            Some("error")
        )
    }

    fn send(title: &str, body: &str, icon: Option<&str>) {
        notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .icon(match icon {
                Some(icon) => icon,
                None => "emblem-new"
            })
            .show()
            .unwrap();
    }
}