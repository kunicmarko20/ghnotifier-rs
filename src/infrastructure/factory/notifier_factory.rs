use crate::infrastructure::noop::noop_notifier::NoopNotifier;
use crate::infrastructure::system::system_notifier::SystemNotifier;
use crate::ui::gui::notifier::Notifier;

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
