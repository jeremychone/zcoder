use crate::core::TuiState;
use crate::view::style;
use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::widgets::Paragraph;

pub struct FooterView;

impl FooterView {
	pub fn render(f: &mut Frame, area: Rect, _state: &TuiState) {
		let footer = Paragraph::new(" [Enter] Send  |  [/q] Quit  |  [Ctrl-c] Quit ").style(style::STL_FOOTER);
		f.render_widget(
			footer,
			area.inner(Margin {
				horizontal: 1,
				vertical: 0,
			}),
		);
	}
}
