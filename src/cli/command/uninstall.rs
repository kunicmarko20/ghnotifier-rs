use structopt::StructOpt;
use super::Command;
use crate::asset::Asset;
use std::path::Path;
use super::Output;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Uninstall {}

impl Command for Uninstall {
    fn execute(&self, output: Box<Output>) {
        output.write("Started uninstalling process.");

        let mut local_data_path = dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");
        let mut desktop_entry_path = local_data_path.clone();

        local_data_path.push(Asset::CONFIG_DIRECTORY);

        if local_data_path.as_path().exists() {
            std::fs::remove_dir_all(local_data_path)
                .expect("Failed to remove local data directory.");
        }

        desktop_entry_path.push(Asset::DESKTOP_ENTRY_PATH);

        if desktop_entry_path.as_path().exists() {
            std::fs::remove_file(desktop_entry_path)
                .expect("Failed to remove desktop entry.");
        }

        let executable_directory_path = dirs::executable_dir()
            .expect("Failed to fetch executable directory.");

        let symlink_path_for_executable = format!(
            "{}/{}",
            executable_directory_path.to_str().unwrap(),
            Asset::EXECUTABLE_NAME,
        );

        if let Ok(_) = std::fs::read_link(Path::new(&symlink_path_for_executable)) {
            std::fs::remove_file(&symlink_path_for_executable)
                .expect("Failed to remove old symlink.");
        }

        output.write("Bye!");
    }
}
