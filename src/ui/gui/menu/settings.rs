use gtk::*;
use arc_guard::ArcGuard;
use crate::infrastructure::file::config::Config;
use crate::ui::gui::window::settings::Settings as SettingsWindow;

pub struct Settings;

impl Settings {
    pub fn create(config: ArcGuard<Config>) -> MenuItem {
        let menu_item = gtk::MenuItem::new_with_label("Settings");

        let config = config.clone();
        menu_item.connect_activate(move |_| {
            SettingsWindow::new(config.clone());
        });

        menu_item
    }
}
