use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

const LOGGER: &'static Logger = &Logger;

pub(crate) struct Logger;

impl Logger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_max_level(LevelFilter::Info);
        log::set_logger(LOGGER)
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
