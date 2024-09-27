use crate::config::Styles;
use ratatui::prelude::Color;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Line, widgets::Widget};

#[derive(Debug, Clone)]
pub struct Button<'a> {
    label: Line<'a>,
    theme: Option<Styles>,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Normal,
    Clicked,
}
/// A button with a label that can be themed.
impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Button {
            label: label.into(),
            theme: None,
            state: State::Normal,
        }
    }

    pub fn theme(mut self, theme: Styles) -> Self {
        self.theme = Some(theme);
        self
    }

    pub const fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
}

impl<'a> Widget for Button<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let _style = self.colors();

        // TODO: use the configured styling
        buf.set_style(area, Style::new().bg(Color::Black).fg(Color::White));

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                // TODO: use the configured styling
                Style::new().bg(Color::Black).fg(Color::White),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                // TODO: use the configured styling
                Style::new().bg(Color::Black).fg(Color::White),
            );
        }
        // render label centered
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}

impl Button<'_> {
    const fn colors(&self) -> Option<Styles> {
        None
    }
}
