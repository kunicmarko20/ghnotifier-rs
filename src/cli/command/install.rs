use super::Command;
use super::Output;
use std::path::PathBuf;
use std::fs::OpenOptions;
use structopt::StructOpt;
use std::io::Write;
use crate::asset::Asset;
use crate::asset;
use std::os::unix::fs;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Install {}

const IMAGE_LOGO: &'static [u8] = include_bytes!("../../../assets/logo.png");
const IMAGE_ISSUE: &'static [u8] = include_bytes!("../../../assets/issue.png");
const IMAGE_PULL_REQUEST: &'static [u8] = include_bytes!("../../../assets/pr.png");
const IMAGE_RELEASE: &'static [u8] = include_bytes!("../../../assets/release.png");

const CONFIG: &'static [u8] = include_bytes!("../../../assets/Config.toml");
const DESKTOP_ENTRY: &'static [u8] = include_bytes!("../../../assets/ghnotifier.desktop");
const EXECUTABLE_PERMISSIONS: u32 = 0o775;

impl Command for Install {
    fn execute(&self, output: Box<Output>) {
        output.write("Started installing process.");

        let local_data_path = dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");

        Self::create_executable_directory(local_data_path.clone());
        Self::create_config_file(local_data_path.clone());
        Self::create_images(local_data_path.clone());
        Self::copy_current_executable_to_executable_directory(local_data_path.clone());
        Self::create_desktop_entry( local_data_path.clone());

        output.write("If you are reading this, all done.");
    }
}

impl Install {
    fn create_executable_directory(mut local_data_path: PathBuf) {
        local_data_path.push(Asset::EXECUTABLE_DIRECTORY);
        std::fs::create_dir_all(local_data_path).unwrap();
    }

    fn create_config_file(mut local_data_path: PathBuf) {
        local_data_path.push(Asset::CONFIG_FILE_PATH);
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(local_data_path) {
            file.write(CONFIG).unwrap();
        }
    }

    fn create_images(mut local_data_path: PathBuf) {
        local_data_path.push(Asset::IMAGE_PATH);
        Self::create_image(local_data_path.clone(), asset::Image::Logo.as_str(), IMAGE_LOGO);
        Self::create_image(local_data_path.clone(), asset::Image::Issue.as_str(), IMAGE_ISSUE);
        Self::create_image(local_data_path.clone(), asset::Image::PullRequest.as_str(), IMAGE_PULL_REQUEST);
        Self::create_image(local_data_path.clone(), asset::Image::Release.as_str(), IMAGE_RELEASE);
    }

    fn create_image(mut local_data_path: PathBuf, image_name: &str, image: &[u8]) {
        local_data_path.push(image_name);
        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(local_data_path) {
            file.write(image).unwrap();
        }
    }

    fn copy_current_executable_to_executable_directory(mut local_data_path: PathBuf) {
        local_data_path.push(Asset::EXECUTABLE_DIRECTORY);

        let new_path_for_current_executable = format!(
            "{}/{}-{}",
            local_data_path.to_str().unwrap(),
            Asset::EXECUTABLE_NAME,
            env!("CARGO_PKG_VERSION"),
        );

        let mut file_content = std::fs::File::create(&new_path_for_current_executable).expect("Failed to create new executable.");

        let mut current_executable = std::fs::File::open(std::env::current_exe().unwrap())
            .expect("Failed to fetch currently executable.");

        std::io::copy(&mut current_executable, &mut file_content)
            .expect("Failed to copy current executable.");

        let metadata = file_content.metadata()
            .expect("Failed get new executable metadata.");;

        let mut permissions = metadata.permissions();

        permissions.set_mode(EXECUTABLE_PERMISSIONS);

        std::fs::set_permissions(&new_path_for_current_executable, permissions)
            .expect("Failed to change permissions on executable.");

        let executable_directory_path = dirs::executable_dir()
            .expect("Failed to fetch executable directory.");

        let symlink_path_for_executable = format!(
            "{}/{}",
            executable_directory_path.to_str().unwrap(),
            Asset::EXECUTABLE_NAME,
        );

        fs::symlink(new_path_for_current_executable, symlink_path_for_executable)
            .expect("Failed to create symlink in system executable folder.");
    }

    fn create_desktop_entry(mut local_data_path: PathBuf) {
        let mut logo_path = local_data_path.clone();
        logo_path.push(Asset::IMAGE_PATH);
        logo_path.push(asset::Image::Logo.as_str());
        local_data_path.push(Asset::DESKTOP_ENTRY_PATH);

        if let Ok(mut file) = OpenOptions::new().create(true).write(true).open(local_data_path) {
            file.write(DESKTOP_ENTRY).unwrap();
            file.write_all(
                format!(
                    r#"
                    Icon={}
                    Exec={} run
                    "#,
                    logo_path.to_str().unwrap(),
                    std::env::current_exe().unwrap().to_str().unwrap()
                ).as_bytes()
            ).unwrap();
        }
    }
}