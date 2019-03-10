pub struct Asset;

impl Asset {
    pub const CONFIG_DIRECTORY: &'static str = "ghnotifier";
    
    pub const LOG_DIRECTORY: &'static str = "ghnotifier/logs";

    pub const EXECUTABLE_NAME: &'static str = "ghnotifier";

    pub const EXECUTABLE_DIRECTORY: &'static str = "ghnotifier/bin";

    pub const IMAGE_PATH: &'static str = "ghnotifier/";

    pub const CONFIG_FILE_PATH: &'static str = "ghnotifier/Config.toml";

    pub const DESKTOP_ENTRY_PATH: &'static str = "applications/ghnotifier.desktop";
}

pub enum Image {
    Logo,
    PullRequest,
    Issue,
    Release,
}

impl Image {
    pub fn as_str(&self) -> &'static str {
        match self {
            Image::Logo => "logo.png",
            Image::PullRequest => "pr.png",
            Image::Issue => "issue.png",
            Image::Release => "release.png",
        }
    }
}
