use std::io::Write;
use std::fs::OpenOptions;
use config::*;
use std::path::PathBuf;
use std::fs;

pub struct Config {
    config: config::Config,
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut inner_config = config::Config::default();
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push("ghnotifier/");

        if !config_path.as_path().exists() {
            return Self::create_default(config_path.clone(), inner_config);
        }

        config_path.push("Config.toml");
        inner_config.merge(config::File::from(config_path.clone())).unwrap();
        Config{config: inner_config, config_path}
    }

    fn create_default(mut config_path: PathBuf, inner_config: config::Config) -> Self {
        fs::create_dir_all(config_path.clone()).unwrap();
        config_path.push("Config.toml");
        let mut config = Config{config: inner_config, config_path};
        config.set("access_token", String::from(""));
        config.set("refresh_time", String::from("10"));
        config.set("quiet_mode", String::from("0"));
        config.save();
        config
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