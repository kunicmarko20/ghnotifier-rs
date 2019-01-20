use std::io::Write;
use std::fs::OpenOptions;
use config::*;
use std::path::PathBuf;

pub struct Config {
    config: config::Config,
    path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut config = config::Config::default();
        let path = PathBuf::from("Config.toml");
        config.merge(config::File::from(path.clone())).unwrap();
        Config{config, path}
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        match &self.config.get::<String>(key) {
            Ok(config) => Ok(config.to_owned()),
            Err(_) => Err(format!("Couldn't get {} from config.", key))
        }
    }

    pub fn set(&mut self, key: &str, value: String) {
        &self.config.set(key, value);
    }

    pub fn save(&mut self) {
        if let Ok(mut file) = OpenOptions::new().write(true).open(&self.path) {
            file.set_len(0).unwrap();
            for (key, value) in self.config.collect().unwrap() {
                file.write_all(
                    format!(
                        "{}=\"{}\"\n",
                        key,
                        value.into_str().unwrap()
                    ).as_bytes()
                ).unwrap();
            }
        }
    }
}