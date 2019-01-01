use super::indicator::Indicator;

pub struct Notifier {
    indicator: Indicator,
    notified_ids: Vec<u64>
}

const GITHUB_API_NOTIFICATIONS: &str = "https://api.github.com/notifications";

impl Notifier {
    pub fn new(indicator: Indicator) -> Notifier {
        Notifier{indicator, notified_ids: Vec::new()}
    }

    pub fn notify(&self) {
        Self::get_notifications();
    }

    fn get_notifications() {
        match reqwest::Client::new()
            .get(GITHUB_API_NOTIFICATIONS)
            .send() {
            Ok(response) => {
                if response.status() != 200 {
                    Self::error();
                    return;
                }
            },
            Err(_) => Self::error()
        }
    }

    fn error() {
        Self::send(
            "Something went wrong",
            "Github didn't respond as expected, check if your access token is correct.",
                Some("error")
        )
    }

    fn send(summary: &str, body: &str, icon: Option<&str>) {
        notify_rust::Notification::new()
            .summary(summary)
            .body(body)
            .icon(match icon {
                Some(icon) => icon,
                None => "emblem-new"
            })
            .show()
            .unwrap();
    }
}