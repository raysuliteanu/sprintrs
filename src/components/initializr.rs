use super::Component;
use crate::{action::Action, config::Config, model, widgets};
use color_eyre::Result;
use ratatui::layout::Constraint::{Fill, Percentage, Ratio};
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
        let [top, middle, bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(area);

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

        frame.render_widget(title, top);

        let [main_left, main_right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(middle);

        let main_left_block = Block::default()
            .borders(Borders::RIGHT)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let mut buf = Vec::new();
        serde_json::to_writer(&mut buf, &self.model).expect("model serialization failed");
        let main_content = Paragraph::new(Text::styled(
            String::from_utf8_lossy(&buf),
            Style::default().fg(Color::Blue),
        ))
        .wrap(Wrap { trim: false })
        .block(main_left_block);

        frame.render_widget(main_content, main_left);

        let main_right_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let main_content = Paragraph::new(Text::styled(
            "main right content coming soon",
            Style::default().fg(Color::Blue),
        ))
        .centered()
        .block(main_right_block);

        frame.render_widget(main_content, main_right);

        let [bottom_layout] = Layout::vertical([Constraint::Percentage(0)]).areas(bottom);

        let [_, button_block_layout, _] =
            Layout::horizontal([Fill(1), Percentage(50), Fill(1)]).areas(bottom_layout);

        let [generate, explore, share] =
            Layout::horizontal([Ratio(1, 3); 3]).areas(button_block_layout);

        let generate_button = button::Button::new("GENERATE");
        let explore_button = button::Button::new("EXPLORE");
        let share_button = button::Button::new("SHARE");

        frame.render_widget(generate_button, generate);
        frame.render_widget(explore_button, explore);
        frame.render_widget(share_button, share);

        Ok(())
    }
}
