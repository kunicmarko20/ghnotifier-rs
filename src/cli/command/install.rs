use super::Command;
use std::path::PathBuf;
use std::fs::OpenOptions;
use structopt::StructOpt;
use std::io::Write;
use crate::asset::Asset;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Install {}

const ICON: &'static [u8] = include_bytes!("../../../assets/icon.png");
const CONFIG: &'static [u8] = include_bytes!("../../../assets/Config.toml");
const DESKTOP_ENTRY: &'static [u8] = include_bytes!("../../../assets/ghnotifier.desktop");

impl Command for Install {
    fn execute(&self) {
        let config_path = dirs::config_dir().unwrap();
        Self::create_config_directory(config_path.clone());
        Self::create_config(config_path.clone());
        Self::create_image(config_path.clone());
        Self::create_desktop_entry( config_path.clone());
    }
}

impl Install {
    fn create_config_directory(mut config_path: PathBuf) {
        config_path.push(Asset::CONFIG_DIRECTORY);
        std::fs::create_dir_all(config_path).unwrap();
    }

    fn create_config(mut config_path: PathBuf) {
        config_path.push(Asset::CONFIG_FILE_PATH);
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(config_path) {
            file.write(CONFIG).unwrap();
        }
    }

    fn create_image(mut config_path: PathBuf) {
        config_path.push(Asset::ICON_PATH);
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(config_path) {
            file.write(ICON).unwrap();
        }
    }

    fn create_desktop_entry(mut config_path: PathBuf) {
        let mut data_path = dirs::data_dir().unwrap();
        data_path.push(Asset::DESKTOP_ENTRY_PATH);
        config_path.push(Asset::ICON_PATH);

        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(data_path) {
            file.write(DESKTOP_ENTRY).unwrap();
            file.write_all(
                format!(
                    r#"
                    Icon={}
                    Exec={} run
                    "#,
                    config_path.to_str().unwrap(),
                    std::env::current_exe().unwrap().to_str().unwrap()
                ).as_bytes()
            ).unwrap();
        }
    }
}