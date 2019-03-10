use libappindicator::{AppIndicator, AppIndicatorStatus};
use crate::ui::asset::Asset;
use crate::ui::asset;
use arc_guard::ArcGuard;
use crate::infrastructure::file::config::Config;
use crate::ui::gui::menu::Menu;

pub struct Indicator {
    app_indicator: AppIndicator
}

const INDICATOR_ID: &str = "Github Notifier RS";

impl Indicator {
    pub fn new(config: ArcGuard<Config>) -> Indicator {
        let mut image_path = dirs::data_local_dir().unwrap();
        image_path.push(Asset::IMAGE_PATH);
        image_path.push(asset::Image::Logo.as_str());

        let mut app_indicator = AppIndicator::new(
            INDICATOR_ID,
            image_path.to_str().expect("Unable to convert path to str.")
        );

        app_indicator.set_menu(&mut Menu::create(config));
        app_indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        app_indicator.set_label("0", "");
        Indicator{app_indicator}
    }

    pub fn change_notification_number(&mut self, label: &str) {
        self.app_indicator.set_label(label, "");
    }
}
