use crate::logger::Logger;

#[derive(Debug)]
pub enum AppError {
    InitializeLogger,
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
    pub fn new() -> Result<Self> {
        Logger::init().unwrap();
        Ok(Self {})
    }

    pub fn run(self) -> Result<()> {
        log::info!("Initialized Phantom Game Engine");
        Ok(())
    }
}
