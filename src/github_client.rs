use reqwest::header::{AUTHORIZATION};
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
        match reqwest::Client::new()
            .get(GITHUB_API_NOTIFICATIONS)
            .header(AUTHORIZATION, String::from("token ") + &self.config.get("access_token")?)
            .send() {
            Ok(mut response) => {
                if response.status() != 200 {
                    return Err(String::from("Github didn't respond as expected, check if your access token is correct."));
                }

                Ok(response.json().unwrap())
            },
            Err(_) => return Err(String::from("Github didn't respond as expected, check if your access token is correct."))
        }
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

