use gtk::*;
use arc_guard::ArcGuard;
use crate::infrastructure::file::config::Config;

pub struct QuietMode;

const QUIET_MODE_OFF_LABEL: &str = "Quiet Mode";
const QUIET_MODE_ON_LABEL: &str = "Quiet Mode ✅";

impl QuietMode {
    pub fn create(config: ArcGuard<Config>) -> MenuItem {
        let label = config.execute(|config| -> &str {
            let config = config.lock().expect("Unable to lock config.");

            if config.get_bool("quiet_mode") {
                return QUIET_MODE_ON_LABEL;
            }

            QUIET_MODE_OFF_LABEL
        });

        let menu_item = MenuItem::new_with_label(label);

        let config = config.arc();
        menu_item.connect_activate(move |menu_item| {
            let mut config = config.lock().expect("Unable to lock config.");

            if config.get_bool("quiet_mode") {
                menu_item.set_label(QUIET_MODE_OFF_LABEL);
                config.set("quiet_mode", String::from("0"));
                config.save();
                return;
            }

            menu_item.set_label(QUIET_MODE_ON_LABEL);
            config.set("quiet_mode", String::from("1"));
            config.save();
        });

        menu_item
    }
}
