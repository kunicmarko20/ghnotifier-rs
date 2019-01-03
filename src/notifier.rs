use super::github_client::Notification;

pub struct Notifier {
    notified_ids: Vec<String>
}

impl Notifier {
    pub fn new() -> Notifier {
        Notifier{notified_ids: Vec::new()}
    }

    pub fn execute(&mut self, notifications: Vec<Notification>) {
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

    pub fn error(body: &str) {
        Self::send(
            "Something went wrong",
            body,
            Some("error")
        )
    }

    pub fn send(title: &str, body: &str, icon: Option<&str>) {
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