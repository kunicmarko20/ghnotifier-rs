use crate::infrastructure::reqwest::github::github_client::GithubClient;
use crate::infrastructure::reqwest::github::notification::Notification;

const NOTIFICATIONS_URL: &str = "https://api.github.com/notifications";

pub struct NotificationClient {
    client: GithubClient,
}

impl NotificationClient {
    pub fn new(client: GithubClient) -> Self {
        NotificationClient{client}
    }

    pub fn notifications(&self) -> Result<Vec<Notification>, String> {
        return self.request_notifications(NOTIFICATIONS_URL)
    }

    fn request_notifications(&self, url: &str) -> Result<Vec<Notification>, String> {
        let mut response = self.client.request(url)?;

        let mut notifications: Vec<Notification> = response.json()
            .map_err(|err| err.to_string())?;

        if let Some(next_page) = self.client.get_next_page(response.headers().clone()) {
            if let Ok(more_notifications) = self.request_notifications(&next_page) {
                notifications.extend(more_notifications);
            }
        }

        Ok(notifications)
    }
}
