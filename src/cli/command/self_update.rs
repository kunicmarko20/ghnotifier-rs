use structopt::StructOpt;
use super::Command;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct SelfUpdate {}

const GITHUB_LATEST_RELEASE: &str = "https://api.github.com/repos/kunicmarko20/ghnotifier-rs/releases/latest";
const BINARY_PERMISSIONS: u32 = 0o775;

impl Command for SelfUpdate {
    fn execute(&self) {
        let mut response = reqwest::get(GITHUB_LATEST_RELEASE)
            .expect("Failed to fetch the current release metadata.");

        let release: Release = response.json()
            .expect("Failed to deserialize the release metadata.");

        if release.tag == env!("CARGO_PKG_VERSION") {
            return;
        }

        let mut binary = reqwest::get(release.binary_download_url())
            .expect("Failed to download the new binary.");

        let current_binary_path = std::env::current_exe()
            .expect("Failed to get the path of the current binary.");

        let current_binary_path = current_binary_path.to_str()
            .expect("Failed to convert the path of the current binary to string.");

        std::fs::remove_file(current_binary_path)
            .expect("Failed to remove the old binary.");

        let mut file_content = std::fs::File::create(current_binary_path)
            .expect("Failed to create the new binary.");

        std::io::copy(&mut binary, &mut file_content)
            .expect("Failed to write the new binary to the disk.");

        let metadata = file_content.metadata()
            .expect("Failed to get the new binary metadata.");;

        let mut permissions = metadata.permissions();

        permissions.set_mode(BINARY_PERMISSIONS);

        std::fs::set_permissions(current_binary_path, permissions)
            .expect("Failed to make the new binary executable.");
    }
}

#[derive(Deserialize)]
struct Release {
    #[serde(rename = "tag_name")]
    pub tag: String,
    assets: Vec<Assets>,
}

impl Release {
    pub fn binary_download_url(&self) -> &String {
        if let Some(asset) = self.assets.first() {
            return &asset.binary_download_url;
        }

        panic!("Binary download URL is missing.");
    }
}

#[derive(Deserialize)]
struct Assets {
    #[serde(rename = "browser_download_url")]
    binary_download_url: String,
}
