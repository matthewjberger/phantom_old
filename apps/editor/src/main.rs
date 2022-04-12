use anyhow::Result;
use phantom::core::Application;

fn main() -> Result<()> {
    Application::new()?.run()?;
    Ok(())
}
