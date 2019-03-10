use crate::ui::gui::notifier::Notifier;
use crate::infrastructure::reqwest::github::notification::Notification;

pub struct NoopNotifier;

impl Notifier for NoopNotifier {
    fn notify(&self, _notification: &Notification) {
    }
    fn error(&self, _body: &str) {
    }
}

impl NoopNotifier {
    pub fn new() -> Self {
        NoopNotifier
    }
}
