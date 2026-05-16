use ratatui::style::{Color, Style};

pub const BKG_APP: Color = Color::Indexed(0);
pub const BKG_PANEL: Color = Color::Indexed(234);
pub const BKG_INPUT: Color = BKG_APP;

pub const BDR_DIVIDER: Color = Color::Indexed(236);

pub const TXT_PRIMARY: Color = Color::Indexed(255);
pub const TXT_SECONDARY: Color = Color::Indexed(252);
pub const TXT_MUTED: Color = Color::Indexed(244);
pub const TXT_DIM: Color = Color::Indexed(240);
pub const TXT_ERROR: Color = Color::Indexed(196);

pub const STL_BKG: Style = Style::new().bg(BKG_APP);
pub const STL_ANSWER: Style = Style::new().fg(TXT_SECONDARY).bg(BKG_PANEL);
pub const STL_INPUT: Style = Style::new().fg(TXT_PRIMARY).bg(BKG_INPUT);
pub const STL_INPUT_WAITING: Style = Style::new().fg(TXT_MUTED).bg(BKG_INPUT);
pub const STL_INPUT_BORDER: Style = Style::new().fg(BDR_DIVIDER).bg(BKG_INPUT);
pub const STL_STATUS_READY: Style = Style::new().fg(TXT_MUTED).bg(BKG_APP);
pub const STL_STATUS_WAITING: Style = Style::new().fg(TXT_MUTED).bg(BKG_APP);
pub const STL_STATUS_ERR: Style = Style::new().fg(TXT_ERROR).bg(BKG_APP);
pub const STL_FOOTER: Style = Style::new().fg(TXT_DIM).bg(BKG_APP);
