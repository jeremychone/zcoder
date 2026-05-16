use crate::tui::core::AppState;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render(f: &mut Frame, state: &AppState) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Length(0), // Header
			Constraint::Min(0),    // Content
			Constraint::Length(1), // Status
			Constraint::Length(3), // Input
			Constraint::Length(1), // Footer
		])
		.split(f.area());

	f.render_widget(Block::new().style(Style::default().bg(Color::Black)), f.area());

	// -- Header

	// -- Content
	let content_text = if let Some(err) = state.last_error() {
		format!("Error: {err}")
	} else if let Some(ans) = state.last_answer() {
		ans.to_string()
	} else {
		"No answer yet. Type a prompt and press Enter.".to_string()
	};

	let content_area = chunks[1].inner(Margin {
		horizontal: 1,
		vertical: 1,
	});
	let content = Paragraph::new(content_text)
		.style(Style::default().fg(Color::Indexed(252)).bg(Color::Indexed(236)))
		.wrap(Wrap { trim: true });
	f.render_widget(content, content_area);

	// -- Status
	let status_style = if state.last_error().is_some() {
		Style::default().fg(Color::Red)
	} else if state.is_waiting() {
		Style::default().fg(Color::Yellow)
	} else {
		Style::default().fg(Color::Green)
	};
	let status = Paragraph::new(format!(" Status: {} ", state.status())).style(status_style);
	f.render_widget(
		status,
		chunks[2].inner(Margin {
			horizontal: 1,
			vertical: 0,
		}),
	);

	// -- Input
	let input_style = if state.is_waiting() {
		Style::default().fg(Color::DarkGray)
	} else {
		Style::default()
	};
	let input = Paragraph::new(state.input())
		.block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
		.style(input_style);
	f.render_widget(
		input,
		chunks[3].inner(Margin {
			horizontal: 1,
			vertical: 0,
		}),
	);

	// -- Footer
	let footer = Paragraph::new(" [Enter] Send  |  [/q] Quit  |  [Ctrl-c] Quit ")
		.style(Style::default().fg(Color::DarkGray));
	f.render_widget(
		footer,
		chunks[4].inner(Margin {
			horizontal: 1,
			vertical: 0,
		}),
	);
}
