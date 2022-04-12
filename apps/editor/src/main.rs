use phantom::core::Application;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    Application::new().unwrap().run().unwrap();
    Ok(())
}
