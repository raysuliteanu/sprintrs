use super::Component;
use crate::{action::Action, config::Config, model, widgets};
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use widgets::button;

pub struct InitializrUi {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    model: model::StartSpringIoModel,
}

impl InitializrUi {
    pub fn new(model: model::StartSpringIoModel) -> Self {
        Self {
            model,
            command_tx: None,
            config: Config::default(),
        }
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
            .split(area);

        let title_block = Block::default()
            .borders(Borders::BOTTOM)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let title_text = vec![
            Span::styled("spring ", Style::default().fg(Color::Green)),
            Span::styled("initializr", Style::default().fg(Color::White)),
        ];

        let title = Paragraph::new(Line::from(title_text))
            .centered()
            .block(title_block);

        frame.render_widget(title, chunks[0]);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let main_block = Block::default()
            .borders(Borders::RIGHT)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let mut buf = Vec::new();
        serde_json::to_writer(&mut buf, &self.model).expect("model serialization failed");
        let main_content = Paragraph::new(Text::styled(
            String::from_utf8_lossy(&buf),
            Style::default().fg(Color::Blue),
        ))
        .block(main_block);

        frame.render_widget(main_content, main_chunks[0]);

        let [button_block] = Layout::vertical([Constraint::Percentage(0)]).areas(chunks[2]);

        let [_, generate, _, explore, _, share, _] = Layout::horizontal([
            Constraint::Percentage(0),
            Constraint::Percentage(20),
            Constraint::Percentage(2),
            Constraint::Percentage(20),
            Constraint::Percentage(2),
            Constraint::Percentage(20),
            Constraint::Percentage(0),
        ])
        .areas(button_block);

        let generate_button = button::Button::new("GENERATE");
        let explore_button = button::Button::new("EXPLORE");
        let share_button = button::Button::new("SHARE");

        frame.render_widget(generate_button, generate);
        frame.render_widget(explore_button, explore);
        frame.render_widget(share_button, share);

        Ok(())
    }
}
