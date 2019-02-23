use structopt::StructOpt;
use super::Command;
use super::super::super::asset::Asset;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Uninstall {}

impl Command for Uninstall {
    fn execute(&self) {
        let mut config_path = dirs::config_dir().unwrap();
        config_path.push(Asset::CONFIG_DIRECTORY);

        if config_path.as_path().exists() {
            std::fs::remove_dir_all(config_path).unwrap();
        }

        let mut data_path = dirs::data_dir().unwrap();
        data_path.push(Asset::DESKTOP_ENTRY_PATH);

        if data_path.as_path().exists() {
            std::fs::remove_file(data_path).unwrap();
        }
    }
}
