extern crate notify_rust;

use notify_rust::Notification;
use super::indicator::Indicator;

pub struct Notifier {
    indicator: Indicator
}

impl Notifier {
    pub fn new(indicator: Indicator) -> Notifier {
        Notifier{indicator}
    }

    pub fn notify(&self) {
        Notification::new()
            .summary("Firefox News")
            .body("This will almost look like a real firefox notification.")
            .icon("emblem-new")
            .show()
            .unwrap();
    }
}