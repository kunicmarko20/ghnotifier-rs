use crate::notifier::Notifier;
use crate::github_client::Notification;

pub struct NoopNotifier;

impl Notifier for NoopNotifier {
    fn notify(&self, _notification: &Notification) {
    }
    fn error(&self, _body: &str) {
    }
}

impl NoopNotifier {
    pub fn new() -> Self {
        NoopNotifier{}
    }
}
