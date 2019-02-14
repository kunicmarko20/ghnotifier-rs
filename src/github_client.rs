use reqwest::hyper_011::{Headers, header::Link, header::RelationType};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use super::config::Config;
use std::sync::{Arc, Mutex};

pub struct GithubClient {
    config: Arc<Mutex<Config>>,
}

const GITHUB_API_NOTIFICATIONS: &str = "https://api.github.com/notifications";

impl GithubClient {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        GithubClient{config}
    }

    pub fn get_notifications(&self) -> Result<Vec<Notification>, String> {
        let notifications = self.request_notifications(GITHUB_API_NOTIFICATIONS);

        if let Err(_) = notifications {
            return Err(String::from("Github didn't respond as expected, check if your access token is correct."));
        }

        Ok(notifications.unwrap())
    }

    fn request_notifications(&self, url: &str) -> Result<Vec<Notification>, ()> {
        let config = &self.config.clone();
        let config = config.lock().unwrap();
        let result = reqwest::Client::new()
            .get(url)
            .header(AUTHORIZATION, String::from("token ") + &config.get("access_token").unwrap())
            .send();

        if let Err(_) = result {
            return Err(());
        }

        let mut response = result.unwrap();

        if response.status() != 200 {
            return Err(());
        }

        let response_as_json = response.json();

        if let Err(_) = response_as_json {
            return Err(());
        }

        let mut notifications: Vec<Notification> = response_as_json.unwrap();

        if let Some(next_page) = Self::get_next_page(response.headers().clone()) {
            if let Ok(more_notifications) = self.request_notifications(&next_page) {
                notifications.extend(more_notifications);
            }
        }

        Ok(notifications)
    }

    fn get_next_page(header_map: HeaderMap) -> Option<String> {
        let headers = Headers::from(header_map);

        let links = headers.get::<Link>()?;

        for link in links.values() {
            if link.rel()? == [RelationType::Next] {
                return Some(link.link().to_owned());
            }
        }

        None
    }
}

#[derive(Deserialize)]
pub struct Notification {
    id: String,
    subject: Subject,
    repository: Repository
}

impl Notification {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.repository.name
    }

    pub fn body(&self) -> &String {
        &self.subject.title
    }
}

#[derive(Deserialize)]
struct Subject {
    title: String
}

#[derive(Deserialize)]
struct Repository {
    name: String
}

