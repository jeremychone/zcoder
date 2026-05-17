use crossterm::event::Event;
use zc_common::event::{Rx, Tx};
use zc_core::ExecStatusEvent;

// region:    --- Tui Event

pub type TuiTx = Tx<TuiEvent>;
pub type TuiRx = Rx<TuiEvent>;

#[derive(Debug, Clone)]
pub enum TuiEvent {
	Term(Event),
	Action(AppActionEvent),
	Exec(ExecStatusEvent),
	Tick,
	DoRedraw,
}

#[derive(Debug, Clone)]
pub enum AppActionEvent {
	Quit,
	RunPrompt(String),
}

// endregion: --- Tui Event

// region:    --- Ping Event

pub type PingTimerTx = Tx<()>;

// endregion: --- Ping Event
