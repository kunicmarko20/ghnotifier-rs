use reqwest::hyper_011::{Headers, header::Link, header::RelationType};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use super::config::Config;

pub struct GithubClient{
    config: Config
}

const GITHUB_API_NOTIFICATIONS: &str = "https://api.github.com/notifications";

impl GithubClient {
    pub fn new() -> Self {
        GithubClient{config: Config::new()}
    }

    pub fn get_notifications(&self) -> Result<Vec<Notification>, String> {
        match self.request_notifications(GITHUB_API_NOTIFICATIONS) {
            Some(notifications) => return Ok(notifications),
            None => return Err(String::from("Github didn't respond as expected, check if your access token is correct."))
        }
    }

    fn request_notifications(&self, url: &str) -> Option<Vec<Notification>> {
        let result = reqwest::Client::new()
            .get(url)
            .header(AUTHORIZATION, String::from("token ") + &self.config.get("access_token").unwrap())
            .send();

        let mut response = result.unwrap();

        if response.status() != 200 {
            return None;
        }

        let mut notifications: Vec<Notification> = response.json().unwrap();

        if let Some(next_page) = Self::get_next_page(response.headers().clone()) {
            if let Some(more_notifications) = self.request_notifications(&next_page) {
                notifications.extend(more_notifications);
            }
        }

        Some(notifications)
    }

    fn get_next_page(header_map: HeaderMap) -> Option<String> {
        let headers = Headers::from(header_map);

        let links = headers.get::<Link>().unwrap();

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

