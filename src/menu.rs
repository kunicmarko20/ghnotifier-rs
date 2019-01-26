use gtk::*;
use super::settings_window::SettingsWindow;
use super::config;

pub struct Menu {
    gtk_menu: gtk::Menu
}

const GITHUB_NOTIFICATIONS: &str = "https://github.com/notifications";
const QUIET_MODE_LABEL: &str = "Quiet Mode";

impl Menu {
    pub fn new() -> Menu {
        let menu = Menu{gtk_menu:gtk::Menu::new()};
        menu.create_menu();
        menu.gtk_menu.show_all();
        menu
    }

    fn create_menu(&self) {
        &self.append_with_callback("Open Notifications", |_| {
            webbrowser::open(GITHUB_NOTIFICATIONS).unwrap();
        });

        &self.append_with_callback("Settings", |_| {
            SettingsWindow::new();
        });

        &self.gtk_menu.append(&gtk::SeparatorMenuItem::new());

        &self.setup_quiet_mode_menu_item();

        &self.gtk_menu.append(&gtk::SeparatorMenuItem::new());

        &self.append_with_callback("Quit", |_| {
            gtk::main_quit();
        });
    }

    fn append_with_callback<F: Fn(&gtk::MenuItem) + 'static>(&self, name: &str, callback: F) {
        let menu_item = gtk::MenuItem::new_with_label(name);
        menu_item.connect_activate(callback);
        &self.gtk_menu.append(&menu_item);
    }

    fn setup_quiet_mode_menu_item(&self) {
        let config = config::Config::new();

        let quiet_mode_label = if config.get("quiet_mode").unwrap() == "1" {
            QUIET_MODE_LABEL.to_string() + " ✅"
        } else {
            QUIET_MODE_LABEL.to_string()
        };

        &self.append_with_callback(&quiet_mode_label, |menu_item| {
            let mut config = config::Config::new();

            if config.get("quiet_mode").unwrap() == "0" {
                menu_item.set_label(&(QUIET_MODE_LABEL.to_owned() + " ✅"));
                config.set("quiet_mode", String::from("1"));
                config.save();
                return
            }

            menu_item.set_label(&QUIET_MODE_LABEL.to_owned());
            config.set("quiet_mode", String::from("0"));
            config.save();
        });
    }

    pub fn inner(&mut self) -> &mut gtk::Menu {
        &mut self.gtk_menu
    }
}