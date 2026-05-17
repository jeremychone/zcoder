use crossterm::event::Event;
use zc_common::ExecStatusEvent;

#[derive(Debug)]
pub enum TuiEvent {
	Term(Event),
	Action(AppActionEvent),
	Exec(ExecStatusEvent),
	Tick,
	DoRedraw,
}

#[derive(Debug)]
pub enum AppActionEvent {
	Quit,
	RunPrompt(String),
}
