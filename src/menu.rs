use gtk::{WidgetExt, MenuShellExt, MenuItemExt};

pub struct Menu {
    gtk_menu: gtk::Menu
}

const GITHUB_NOTIFICATIONS: &str = "https://github.com/notifications";

impl Menu {
    pub fn new() -> Menu {
        let menu = Menu{gtk_menu:gtk::Menu::new()};
        menu.create_menu();
        menu.gtk_menu.show_all();
        menu
    }

    fn create_menu(&self) {
        &self.append_with_callback("Open Notifications", |_| {
            webbrowser::open(GITHUB_NOTIFICATIONS);
        });

        &self.append_with_callback("Settings", |_| {
        });

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

    pub fn inner(&mut self) -> &mut gtk::Menu {
        &mut self.gtk_menu
    }
}