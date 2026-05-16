use crate::tui::core::AppState;
use crate::tui::view::style;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Position};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};

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

	f.render_widget(Block::new().style(style::STL_BKG), f.area());

	// -- Header

	// -- Content
	let content_text = if let Some(err) = state.last_error() {
		format!("Error: {err}")
	} else if let Some(ans) = state.last_answer() {
		ans.to_string()
	} else {
		"No answer yet, enter prompt".to_string()
	};

	let content_area = chunks[1].inner(Margin {
		horizontal: 1,
		vertical: 1,
	});
	let content = Paragraph::new(content_text)
		.block(Block::new().style(style::STL_ANSWER).padding(Padding::new(2, 2, 0, 0)))
		.style(style::STL_ANSWER)
		.wrap(Wrap { trim: true });
	f.render_widget(content, content_area);

	// -- Status
	let status_style = style::style_status(state.last_error().is_some(), state.is_waiting());
	let status = Paragraph::new(format!(" Status: {} ", state.status())).style(status_style);
	f.render_widget(
		status,
		chunks[2].inner(Margin {
			horizontal: 1,
			vertical: 0,
		}),
	);

	// -- Input
	let input_area = chunks[3].inner(Margin {
		horizontal: 1,
		vertical: 0,
	});
	let input_style = style::style_input(state.is_waiting());
	f.render_widget(Block::new().style(style::STL_INPUT), chunks[3]);
	let input_text = format!("> {}", state.input());
	let input = Paragraph::new(input_text)
		.block(Block::default().borders(Borders::TOP | Borders::BOTTOM).style(style::STL_INPUT_BORDER))
		.style(input_style);
	f.render_widget(input, input_area);

	if !state.is_waiting() && input_area.width > 0 && input_area.height > 1 {
		let cursor_x = input_area
			.x
			.saturating_add(2)
			.saturating_add(state.input().chars().count() as u16)
			.min(input_area.x.saturating_add(input_area.width.saturating_sub(1)));
		let cursor_y = input_area
			.y
			.saturating_add(1)
			.min(input_area.y.saturating_add(input_area.height.saturating_sub(1)));
		f.set_cursor_position(Position::new(cursor_x, cursor_y));
	}

	// -- Footer
	let footer = Paragraph::new(" [Enter] Send  |  [/q] Quit  |  [Ctrl-c] Quit ")
		.style(style::STL_FOOTER);
	f.render_widget(
		footer,
		chunks[4].inner(Margin {
			horizontal: 1,
			vertical: 0,
		}),
	);
}
