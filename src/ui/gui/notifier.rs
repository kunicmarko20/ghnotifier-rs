use crate::infrastructure::reqwest::github::notification::Notification;

pub trait Notifier {
    fn notify(&self, notification: &Notification);
    fn error(&self, body: &str);
}
