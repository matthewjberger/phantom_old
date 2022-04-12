use crate::logger::Logger;
use image::io::Reader;
use thiserror::Error;
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{self, EventLoop},
    window::{Icon, Window, WindowBuilder},
};

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
    pub fn run(config: Config) -> Result<()> {
        initialize_logger()?;

        let (event_loop, window) = create_window(config)?;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    // update
                    // render
                }
                _ => (),
            }
        });
    }
}

fn initialize_logger() -> Result<()> {
    if let Err(error) = Logger::init() {
        return Err(AppError::InitializeLogger(error));
    }
    log::info!("Initialized Phantom Game Engine");
    Ok(())
}

fn create_window(config: Config) -> Result<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title(config.title.to_string())
        .with_inner_size(PhysicalSize::new(config.width, config.height));

    // if let Some(icon_path) = config.icon.as_ref() {
    //     let image = Reader::open(icon_path)?.decode()?.into_rgba8();
    //     let (width, height) = image.dimensions();
    //     let icon = Icon::from_rgba(image.into_raw(), width, height)?;
    //     window_builder = window_builder.with_window_icon(Some(icon));
    // }

    let window = window_builder.build(&event_loop).unwrap();

    Ok((event_loop, window))
}
