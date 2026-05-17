use super::app_event_handlers::{handle_app_action, handle_exec_status, handle_term_event};
use super::{TuiEvent, TuiState};
use crate::view;
use crate::{Error, Result};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc::{Receiver, Sender};
use zc_core::ExecutorTx;

pub async fn run_ui_loop(
	mut terminal: DefaultTerminal,
	mut app_rx: Receiver<TuiEvent>,
	app_tx: Sender<TuiEvent>,
	executor_tx: ExecutorTx,
	initial_prompt: Option<String>,
) -> Result<()> {
	let mut state = TuiState::new(initial_prompt);

	loop {
		terminal.draw(|f| view::render(f, &state))?;

		let app_event = app_rx.recv().await.ok_or_else(|| Error::custom("App event channel closed"))?;

		match app_event {
			TuiEvent::Term(term_event) => {
				handle_term_event(&mut state, &app_tx, term_event).await;
			}

			TuiEvent::Action(action) => {
				if handle_app_action(&mut state, &executor_tx, action).await? {
					break;
				}
			}

			TuiEvent::Exec(status) => {
				handle_exec_status(&mut state, status);
			}

			TuiEvent::Tick | TuiEvent::DoRedraw => (),
		}
	}

	Ok(())
}
