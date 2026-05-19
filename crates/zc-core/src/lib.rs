// region:    --- Modules

mod derive_aliases;
mod error;
mod executor;
mod model;

use derive_aliases::*;
pub use error::{Error, Result};
pub use event::{ExecAction, ExecEvent};
pub use executor::{Executor, ExecutorConfig};

pub mod event;

// endregion: --- Modules
