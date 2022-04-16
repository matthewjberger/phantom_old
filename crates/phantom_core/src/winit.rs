use image::{io::Reader, ImageError};
use winit::{
    dpi::PhysicalSize,
    error::OsError,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, Window, WindowBuilder},
};

use crate::{state, Config, State, StateData, StateMachine};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create the state machine")]
    CreateStateMachine(#[from] state::Error),
    #[error("Failed to create the window")]
    CreateWindow(#[from] OsError),
    #[error("Failed to load image")]
    LoadImage(#[from] std::io::Error),
    #[error("Failed to decode image")]
    DecodeImage(#[from] ImageError),
    #[error("Failed to load icon")]
    LoadIcon(#[from] winit::window::BadIcon),
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub fn run_winit(
    config: Config,
    initial_state: impl State<(), (), ()> + 'static,
) -> Result<()> {
    let mut state_machine = StateMachine::new(initial_state);
    state_machine.start(StateData::new(&mut (), &mut ()))?;

    let (event_loop, _window) = create_window(config)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {
                state_machine.update(StateData::new(&mut (), &mut ()));
            }
            _ => (),
        }
    });
}

fn create_window(config: Config) -> Result<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new();

    let mut window_builder = WindowBuilder::new()
        .with_title(config.title.to_string())
        .with_inner_size(PhysicalSize::new(config.width, config.height));

    if let Some(icon_path) = config.icon.as_ref() {
        let image = Reader::open(icon_path)
            .map_err(Error::LoadImage)?
            .decode()
            .map_err(Error::DecodeImage)?
            .into_rgba8();
        let (width, height) = image.dimensions();
        let icon = Icon::from_rgba(image.into_raw(), width, height)
            .map_err(Error::LoadIcon)?;
        window_builder = window_builder.with_window_icon(Some(icon));
    }

    let window = window_builder.build(&event_loop)?;

    Ok((event_loop, window))
}
