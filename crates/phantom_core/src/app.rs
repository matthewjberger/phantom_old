use crate::logger::Logger;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to initialize the logger.")]
    InitializeLogger(log::SetLoggerError),
}

type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub is_fullscreen: bool,
    pub title: String,
    pub icon: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            is_fullscreen: false,
            title: "Phantom Game Engine".to_string(),
            icon: None,
        }
    }
}

pub struct Application {
    // state_machine: StateMachine,
}

impl Application {
    pub fn run(_config: Config) -> Result<()> {
        Self::initialize_logger()?;
        Ok(())
    }

    fn initialize_logger() -> Result<()> {
        if let Err(error) = Logger::init() {
            return Err(AppError::InitializeLogger(error));
        }
        log::info!("Initialized Phantom Game Engine");
        Ok(())
    }
}
