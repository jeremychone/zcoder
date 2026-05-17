// region:    --- Modules

mod error;
mod executor;

pub use error::{Error, Result};
pub use event::{ExecActionEvent, ExecStatusEvent};
pub use executor::{Executor, ExecutorConfig};

pub mod event;

// endregion: --- Modules
