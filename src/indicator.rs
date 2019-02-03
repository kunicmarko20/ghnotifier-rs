use libappindicator::{AppIndicator, AppIndicatorStatus};
use super::menu::Menu;
use std::io::Write;
use std::fs::OpenOptions;
use std::path::PathBuf;

pub struct Indicator {
    app_indicator: AppIndicator
}

const INDICATOR_ID: &str = "Github Notifier RS";
const ICON: &'static [u8] = include_bytes!("icon.png");

impl Indicator {
    pub fn new(mut menu: Menu) -> Indicator {
        let mut image_path = dirs::config_dir().unwrap();
        image_path.push("ghnotifier/icon.png");

        if !image_path.as_path().exists() {
            Self::create_image(image_path.clone());
        }

        let mut app_indicator = AppIndicator::new(
            INDICATOR_ID,
            image_path.to_str().unwrap()
        );

        app_indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        app_indicator.set_label("0", "");
        app_indicator.set_menu(menu.inner());
        Indicator{app_indicator}
    }

    fn create_image(image_path: PathBuf) {
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(image_path) {
            file.write(ICON).unwrap();
        }
    }

    pub fn change_notification_number(&mut self, label: &str) {
        &self.app_indicator.set_label(label, "");
    }
}
