use crate::{logger::Logger, winit::run_winit, State};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to initialize the logger")]
    InitializeLogger(log::SetLoggerError),

    #[error("Failed to initialize winit")]
    InitializeWinit(#[from] crate::winit::Error),
}

type Result<T, E = Error> = std::result::Result<T, E>;

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

pub struct Application;

impl Application {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn run(
        self,
        config: Config,
        initial_state: impl State<(), (), ()> + 'static,
    ) -> Result<()> {
        initialize_logger()?;
        run_winit(config, initial_state).map_err(Error::InitializeWinit)
    }
}

fn initialize_logger() -> Result<()> {
    if let Err(error) = Logger::init() {
        return Err(Error::InitializeLogger(error));
    }
    log::info!("Initialized Phantom Game Engine");
    Ok(())
}
