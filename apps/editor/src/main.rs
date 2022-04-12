use anyhow::Result;
use phantom::core::{Application, Config, State, StateData, Transition};

#[derive(Default)]
struct EditorState;

impl State<(), (), ()> for EditorState {
    fn label(&self) -> String {
        "Editor".to_string()
    }

    fn update(&mut self, _: StateData<'_, (), ()>) -> Transition<(), (), ()> {
        log::info!("Editor is running!");
        Transition::None
    }
}

fn main() -> Result<()> {
    Application::run(Config::default(), EditorState::default())?;
    Ok(())
}
