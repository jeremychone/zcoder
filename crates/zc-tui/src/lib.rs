// region:    --- Modules

mod core;
mod error;
mod view;

pub use core::start_tui;
pub use error::{Error, Result};

pub mod event;

// endregion: --- Modules
