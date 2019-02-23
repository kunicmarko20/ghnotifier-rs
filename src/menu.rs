use gtk::*;
use super::settings_window::SettingsWindow;
use super::config;
use std::sync::{Arc, Mutex};

pub struct Menu {
    gtk_menu: gtk::Menu,
    config: Arc<Mutex<config::Config>>
}

const GITHUB_NOTIFICATIONS: &str = "https://github.com/notifications";
const QUIET_MODE_LABEL: &str = "Quiet Mode";

impl Menu {
    pub fn new(config: Arc<Mutex<config::Config>>) -> Menu {
        let mut menu = Menu{gtk_menu:gtk::Menu::new(), config};
        menu.create_menu();
        menu.gtk_menu.show_all();
        menu
    }

    fn create_menu(&mut self) {
        &self.append_with_callback("Open Notifications", |_| {
            webbrowser::open(GITHUB_NOTIFICATIONS).unwrap();
        });

        let config = self.config.clone();
        &self.append_with_callback("Settings", move|_| {
            SettingsWindow::new(config.clone());
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

    fn setup_quiet_mode_menu_item(&mut self) {
        let config = self.config.clone();
        let config = config.lock().unwrap();
        let quiet_mode_label = Self::resolve_quite_mode_label(config.get("quiet_mode").unwrap());

        let config = self.config.clone();
        &self.append_with_callback(&quiet_mode_label, move |menu_item| {
            let mut config = config.lock().unwrap();
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

    fn resolve_quite_mode_label(config: String) -> String {
        if config == "1" {
            return QUIET_MODE_LABEL.to_string() + " ✅";
        }

        QUIET_MODE_LABEL.to_string()
    }

    pub fn inner(&mut self) -> &mut gtk::Menu {
        &mut self.gtk_menu
    }
}