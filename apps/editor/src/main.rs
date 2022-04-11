use phantom::core::Logger;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    if let Err(err) = Logger::init() {
        return Err(format!("failed to initialize logger: {}", err).into());
    }
    log::info!("Hello, world!");
    Ok(())
}
