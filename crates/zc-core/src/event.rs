// region:    --- ExecutorAction

#[derive(Debug)]
pub enum ExecActionEvent {
	RunPrompt(String),
}

pub type ExecutorActionRx = zc_common::event::Rx<ExecActionEvent>;
pub type ExecutorTx = zc_common::event::Tx<ExecActionEvent>;

// endregion: --- ExecutorAction

// region:    --- ExecStatus

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ExecStatusEvent {
	RunStart,
	RunEnd,
	RunResult(String),
	RunError(String),
}

pub type ExecutorStatusRx = zc_common::event::Rx<ExecStatusEvent>;
pub type ExecutorStatusTx = zc_common::event::Tx<ExecStatusEvent>;

// endregion: --- ExecStatus
