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

        inner_config.merge(config::File::from(config_path.clone()))
            .expect("Unable to merge config.");

        Config{config: inner_config, config_path}
    }

    pub fn get(&self, key: &str) -> String {
        self.config.get::<String>(key)
            .expect(&format!("Unbale to get {} from config.", key))
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.config.set(key, value)
            .expect(&format!("Unable to set {}.", key));
    }

    pub fn save(&mut self) {
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(&self.config_path) {
            file.set_len(0).expect("Unable to truncate file.");
            for (key, value) in self.config.collect().expect("Unable to collect config.") {
                file.write_all(
                    format!(
                        "{}=\"{}\"\n",
                        key,
                        value.into_str().expect("Unable to convert config value to String.")
                    ).as_bytes()
                ).expect("Unable to write to Config file");
            }
        }
    }
}