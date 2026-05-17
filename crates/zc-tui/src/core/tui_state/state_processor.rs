use super::TuiState;

pub struct StateProcessor;

impl StateProcessor {
	pub fn start_prompt_run(state: &mut TuiState) {
		state.clear_input();
		state.set_waiting(true);
		state.set_last_error(None);
	}

	pub fn apply_run_start(state: &mut TuiState) {
		state.set_status("Sending to AI...".to_string());
	}

	pub fn apply_run_end(state: &mut TuiState) {
		state.set_waiting(false);
		state.set_status("Idle".to_string());
	}

	pub fn apply_run_result(state: &mut TuiState, answer: String) {
		state.set_last_answer(Some(answer));
	}

	pub fn apply_run_error(state: &mut TuiState, error: String) {
		state.set_last_error(Some(error));
	}
}
