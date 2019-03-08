use std::io::Write;
use std::fs::OpenOptions;
use config::*;
use super::asset::Asset;
use std::path::PathBuf;

pub struct Config {
    config: config::Config,
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut inner_config = config::Config::default();
        let mut config_path = dirs::data_local_dir().unwrap();
        config_path.push(Asset::CONFIG_FILE_PATH);

        inner_config.merge(config::File::from(config_path.clone())).unwrap();
        Config{config: inner_config, config_path}
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
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(&self.config_path) {
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