use log::LevelFilter;
use chrono::Utc;
use crate::infrastructure::logger::Logger;
use crate::ui::asset::Asset;

pub struct FileLogger;

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        error!("{}", message);
    }
}

impl FileLogger {
    pub fn new() -> impl Logger {
        let mut local_data_path = dirs::data_local_dir()
            .expect("Failed to fetch local data directory.");

        local_data_path.push(
            format!(
                "{}/{}.log",
                Asset::LOG_DIRECTORY,
                Utc::now().to_rfc3339()
            )
        );

        simple_logging::log_to_file(local_data_path, LevelFilter::Error)
            .expect("Unable to use file logging.");
        FileLogger
    }
}
