use self::noop_notifier::NoopNotifier;
use self::system_notifier::SystemNotifier;
use crate::github_client::Notification;

pub trait Notifier {
    fn notify(&self, notification: &Notification);
    fn error(&self, body: &str);
}

pub struct NotifierFactory;

impl NotifierFactory {
    pub fn new() -> Self {
        NotifierFactory
    }

    pub fn from_arg(&self, quiet: bool) -> Box<Notifier> {
        if quiet {
            return Box::new(NoopNotifier::new());
        }

        Box::new(SystemNotifier::default())
    }
}

mod noop_notifier;
mod system_notifier;
