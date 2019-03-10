use gtk::*;

pub struct OpenNotifications;

const GITHUB_NOTIFICATIONS: &str = "https://github.com/notifications";

impl OpenNotifications {
    pub fn create() -> MenuItem {
        let menu_item = MenuItem::new_with_label("Open Notifications");

        menu_item.connect_activate(|_| {
            webbrowser::open(GITHUB_NOTIFICATIONS)
                .expect("Unable to open Notifications page.");
        });

        menu_item
    }
}
