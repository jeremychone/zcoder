#![allow(dead_code, unused_imports)]

// region:    --- Modules

mod app_event_handlers;
mod event;
mod ping_timer;
mod term_reader;
mod tui_impl;
mod tui_loop;
mod tui_state;

pub mod types;

pub use event::{AppActionEvent, TuiEvent};
pub use ping_timer::{PingTimerTx, start_ping_timer};
pub use tui_impl::{AppTx, start_tui};
pub use tui_state::TuiState;

// endregion: --- Modules
