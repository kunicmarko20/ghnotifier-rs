use structopt::StructOpt;
use super::Command;
use std::os::unix::fs;
use std::os::unix::fs::PermissionsExt;
use super::Output;
use crate::ui::asset::Asset;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct SelfUpdate {}

const GITHUB_LATEST_RELEASE: &str = "https://api.github.com/repos/kunicmarko20/ghnotifier-rs/releases/latest";
const EXECUTABLE_PERMISSIONS: u32 = 0o775;

impl Command for SelfUpdate {
    fn execute(&self, output: Box<Output>) {
        output.write("Started self update process.");

        let mut response = reqwest::get(GITHUB_LATEST_RELEASE)
            .expect("Failed to fetch the current release metadata.");

        let release: Release = response.json()
            .expect("Failed to deserialize the release metadata.");

        if release.tag == env!("CARGO_PKG_VERSION") {
            return;
        }

        let mut executable = reqwest::get(release.executable_download_url())
            .expect("Failed to download new executable.");

        let mut local_data_path = dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");

        local_data_path.push(Asset::EXECUTABLE_DIRECTORY);

        let path_for_new_executable = format!(
            "{}/{}-{}",
            local_data_path.to_str().unwrap(),
            Asset::EXECUTABLE_NAME,
            release.tag,
        );

        let mut file_content = std::fs::File::create(&path_for_new_executable)
            .expect("Failed to create new executable file.");

        std::io::copy(&mut executable, &mut file_content)
            .expect("Failed to write new executable to the disk.");

        let metadata = file_content.metadata()
            .expect("Failed to get new executable metadata.");

        let mut permissions = metadata.permissions();

        permissions.set_mode(EXECUTABLE_PERMISSIONS);

        std::fs::set_permissions(&path_for_new_executable, permissions)
            .expect("Failed to change permissions on executable.");

        let executable_directory_path = dirs::executable_dir()
            .expect("Failed to fetch local data directory.");

        let symlink_path_for_executable = format!(
            "{}/{}",
            executable_directory_path.to_str().unwrap(),
            Asset::EXECUTABLE_NAME,
        );

        if let Ok(_) = std::fs::read_link(std::path::Path::new(&symlink_path_for_executable)) {
            std::fs::remove_file(&symlink_path_for_executable)
                .expect("Failed to remove old symlink.");
        }

        fs::symlink(path_for_new_executable, symlink_path_for_executable)
            .expect("Failed to create symlink in system executable folder.");

        output.write(&format!("If you are reading this, all done. Your new version is: {}", release.tag));
    }
}

#[derive(Deserialize)]
struct Release {
    #[serde(rename = "tag_name")]
    pub tag: String,
    assets: Vec<Assets>,
}

impl Release {
    pub fn executable_download_url(&self) -> &String {
        if let Some(asset) = self.assets.first() {
            return &asset.executable_download_url;
        }

        panic!("Download URL missing.");
    }
}

#[derive(Deserialize)]
struct Assets {
    #[serde(rename = "browser_download_url")]
    executable_download_url: String,
}
