use gtk::*;
use arc_guard::ArcGuard;
use crate::infrastructure::file::config::Config;
use crate::ui::gui::menu::open_notifications::OpenNotifications;
use crate::ui::gui::menu::quiet_mode::QuietMode;
use crate::ui::gui::menu::quit::Quit;
use crate::ui::gui::menu::settings::Settings;

pub struct Menu;

impl Menu {
    pub fn create(config: ArcGuard<Config>) -> gtk::Menu {
        let menu = gtk::Menu::new();

        menu.append(&OpenNotifications::create());
        menu.append(&Settings::create(config.clone()));
        menu.append(&gtk::SeparatorMenuItem::new());
        menu.append(&QuietMode::create(config.clone()));
        menu.append(&gtk::SeparatorMenuItem::new());
        menu.append(&Quit::create());

        menu.show_all();
        menu
    }
}

mod open_notifications;
mod quiet_mode;
mod quit;
mod settings;
