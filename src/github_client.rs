use reqwest::hyper_011::{Headers, header::Link, header::RelationType};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use super::config::Config;
use arc_guard::ArcGuard;

pub struct GithubClient {
    config: ArcGuard<Config>,
}

const GITHUB_API_NOTIFICATIONS: &str = "https://api.github.com/notifications";

impl GithubClient {
    pub fn new(config: ArcGuard<Config>) -> Self {
        GithubClient{config}
    }

    pub fn get_notifications(&self) -> Result<Vec<Notification>, String> {
        return self
            .request_notifications(GITHUB_API_NOTIFICATIONS)
            .map_err(|_| String::from("Github didn't respond as expected, check if your access token is correct."));
    }

    fn request_notifications(&self, url: &str) -> Result<Vec<Notification>, ()> {
        let authorization_header = self.config.execute(|config| -> String {
            let config = config.lock().unwrap();
            String::from("token ") + &config.get("access_token").unwrap()
        });

        let mut response = reqwest::Client::new()
            .get(url)
            .header(AUTHORIZATION, authorization_header)
            .send()
            .map_err(|_| ())?;

        if response.status() != 200 {
            return Err(());
        }

        let mut notifications: Vec<Notification> = response.json().map_err(|_| ())?;

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

