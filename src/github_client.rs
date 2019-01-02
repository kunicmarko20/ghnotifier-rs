use std::error::Error;
use reqwest::header::{AUTHORIZATION};

pub struct GithubClient;

const GITHUB_API_NOTIFICATIONS: &str = "https://api.github.com/notifications";

impl GithubClient {
    pub fn get_notifications() -> Result<Vec<Notification>, String> {
        match reqwest::Client::new()
            .get(GITHUB_API_NOTIFICATIONS)
            .header(AUTHORIZATION, "token xxx")
            .send() {
            Ok(mut response) => {
                if response.status() != 200 {
                    return Err(String::from("Something went wrong."));
                }

                Ok(response.json().unwrap())
            },
            Err(_) => return Err(String::from("Some`thing went wrong."))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct Subject {
    title: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    name: String
}

