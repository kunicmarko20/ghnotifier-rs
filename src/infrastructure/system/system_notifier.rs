use std::path::PathBuf;
use crate::ui::gui::notifier::Notifier;
use crate::infrastructure::reqwest::github::notification::{Notification, NotificationType};
use crate::ui::asset::Asset;
use crate::ui::asset;

pub struct SystemNotifier {
    path_to_icons: PathBuf
}

impl Notifier for SystemNotifier {
    fn notify(&self, notification: &Notification) {
        let mut path_to_icon = self.path_to_icons.clone();
        path_to_icon.push(Self::type_to_image(notification.notification_type()).as_str());

        self.send(
            notification.title(),
            notification.body(),
            path_to_icon.to_str().expect("Unable to print path of notification icon.")
        )
    }

    fn error(&self, body: &str) {
        self.send(
            "Something went wrong",
            body,
            "error"
        )
    }
}

impl SystemNotifier {
    pub fn default() -> Self {
        let mut path_to_icons= dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");
        path_to_icons.push(Asset::IMAGE_PATH);
        SystemNotifier {path_to_icons}
    }

    fn type_to_image(notification_type: &NotificationType) -> asset::Image {
        match notification_type {
            NotificationType::Release => asset::Image::Release,
            NotificationType::Issue => asset::Image::Issue,
            NotificationType::PullRequest => asset::Image::PullRequest,
        }
    }

    fn send(&self, title: &str, body: &str, icon: &str) {
        notify_rust::Notification::new()
            .appname("Github Notifier")
            .summary(title)
            .body(body)
            .image_path(icon)
            .show()
            .expect("Unable to send notification.");
    }
}
