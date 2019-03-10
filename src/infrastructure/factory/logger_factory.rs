use crate::infrastructure::logger::Logger;
use crate::infrastructure::file::file_logger::FileLogger;

pub struct LoggerFactory;

impl LoggerFactory {
    pub fn create() -> Box<Logger> {
        Box::new(FileLogger::new())
    }
}
