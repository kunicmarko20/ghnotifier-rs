use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::env;
use super::menu::Menu;

pub struct Indicator {
    app_indicator: AppIndicator
}

const INDICATOR_ID: &str = "Github Notifier RS";

impl Indicator {
    pub fn new() -> Indicator {
        let mut app_indicator = AppIndicator::new(INDICATOR_ID, &format!(
            "{}{}",
            env::current_dir().unwrap().to_str().unwrap(),
            "/src/gh.png"
        ));
        app_indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        app_indicator.set_label("0", "");
        Indicator{app_indicator}
    }

    pub fn set_menu(&mut self, mut menu: Menu) {
        self.app_indicator.set_menu(menu.inner())
    }
}
