use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct InitializrUi {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl InitializrUi {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for InitializrUi {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "spring initializr",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        frame.render_widget(title, chunks[0]);

        let main_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let main_content = Paragraph::new(Text::styled(
            "main content goes here",
            Style::default().fg(Color::Blue),
        ))
        .block(main_block);

        frame.render_widget(main_content, chunks[1]);

        let footer_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(chunks[2]);

        let footer_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let footer_content = Paragraph::new(Text::styled(
            "buttons will go here",
            Style::default().fg(Color::Red),
        ))
        .block(footer_block);

        frame.render_widget(footer_content, footer_chunk[0]);

        Ok(())
    }
}
