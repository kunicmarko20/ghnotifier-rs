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

    pub fn notify(&mut self) {
        match GithubClient::get_notifications() {
            Ok(notifications) => self.ok(notifications),
            Err(_) => Self::error()
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
    }

    fn error() {
        Self::send(
            "Something went wrong",
            "Github didn't respond as expected, check if your access token is correct.",
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