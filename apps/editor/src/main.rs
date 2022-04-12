use anyhow::Result;
use phantom::core::{Application, Config};

fn main() -> Result<()> {
    Application::run(Config::default())?;
    Ok(())
}
