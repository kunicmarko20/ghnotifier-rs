pub struct Config{
    config: config::Config
}

impl Config {
    pub fn new() -> Self {
        let mut config = config::Config::default();
        config.merge(config::File::with_name("Config")).unwrap();
        Config{config}
    }

    pub fn get(&self, name: &str) -> Result<String, String> {
        match &self.config.get::<String>(name) {
            Ok(config) => Ok(config.to_owned()),
            Err(_) => Err(format!("Couldn't get {} from config.", name))
        }
    }
}