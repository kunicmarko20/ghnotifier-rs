use reqwest::hyper_011::{Headers, header::Link, header::RelationType};
use reqwest::header::{AUTHORIZATION, HeaderMap};
use arc_guard::ArcGuard;
use reqwest::Response;
use crate::infrastructure::file::config::Config;

pub struct GithubClient {
    config: ArcGuard<Config>,
}

impl GithubClient {
    pub fn new(config: ArcGuard<Config>) -> Self {
        GithubClient { config }
    }

    pub fn request(&self, url: &str) -> Result<Response, String> {

        let authorization_header = self.config.execute(|config| -> String {
            let config = config.lock().expect("Unable to lock config.");
            format!("token {}", &config.get_string("access_token"))
        });

        let mut response = reqwest::Client::new()
            .get(url)
            .header(AUTHORIZATION, authorization_header)
            .send()
            .map_err(|err| err.to_string())?;


        if response.status() != 200 {
            return Err(
                format!(
                    "Status code was {}. Response message: {}",
                    response.status(),
                    response.text().expect("Unable to grab response content.")
                )
            );
        }

        Ok(response)
    }

    pub fn get_next_page(&self, header_map: HeaderMap) -> Option<String> {
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
