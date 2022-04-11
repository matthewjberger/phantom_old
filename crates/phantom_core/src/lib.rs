mod logger;
mod state;

pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

pub use self::{logger::*, state::*};
