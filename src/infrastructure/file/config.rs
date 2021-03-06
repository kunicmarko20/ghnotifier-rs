use std::io::Write;
use std::fs::OpenOptions;
use config::Config as ExternalConfig;
use config::*;
use std::path::PathBuf;
use crate::ui::asset::Asset;

pub struct Config {
    config: ExternalConfig,
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut inner_config = ExternalConfig::default();
        let mut config_path = dirs::data_local_dir().unwrap();
        config_path.push(Asset::CONFIG_FILE_PATH);

        inner_config.merge(File::from(config_path.clone()))
            .expect("Unable to merge config.");

        Config{config: inner_config, config_path}
    }

    pub fn get_string(&self, key: &str) -> String {
        self.config.get::<String>(key)
            .expect(&format!("Unbale to get {} as String.", key))
    }

    pub fn get_u64(&self, key: &str) -> u64 {
        self.config.get::<u64>(key)
            .expect(&format!("Unbale to get {} as u64.", key))
    }

    pub fn get_bool(&self, key: &str) -> bool {
        match self.get_string(key).as_ref() {
            "0" => false,
            "1" => true,
            _ => panic!(format!("Unbale to get {} as bool.", key))
        }
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