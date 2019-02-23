use libappindicator::{AppIndicator, AppIndicatorStatus};
use super::menu::Menu;
use super::asset::Asset;

pub struct Indicator {
    app_indicator: AppIndicator
}

const INDICATOR_ID: &str = "Github Notifier RS";

impl Indicator {
    pub fn new(mut menu: Menu) -> Indicator {
        let mut image_path = dirs::config_dir().unwrap();
        image_path.push(Asset::ICON_PATH);

        let mut app_indicator = AppIndicator::new(
            INDICATOR_ID,
            image_path.to_str().unwrap()
        );

        app_indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        app_indicator.set_label("0", "");
        app_indicator.set_menu(menu.inner());
        Indicator{app_indicator}
    }


    pub fn change_notification_number(&mut self, label: &str) {
        &self.app_indicator.set_label(label, "");
    }
}
