mod client;
mod common;
mod config;
mod model;
mod tui;

use crate::model::StartSpringIoModel;
use crate::tui::Event;
use clap::Parser;
use crossterm::event::KeyCode;
use log::debug;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 4.0)]
    tick_rate: f64,
    #[arg(short, long, default_value_t = 30.0)]
    frame_rate: f64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cli = Args::parse();

    debug!("initializing client");
    let client = client::Client::new()?;
    let model = client.initialize().await?;

    debug!("initializing ui");
    let mut tui = tui::Tui::new()?
        .tick_rate(cli.tick_rate) // 4 ticks per second
        .frame_rate(cli.frame_rate); // 30 frames per second

    tui.enter()?; // Starts event handler, enters raw mode, enters alternate screen

    debug!("starting event loop");
    loop {
        tui.draw(|f| {
            // Deref allows calling `tui.terminal.draw`
            ui(f, &model);
        })?;

        // `tui.next().await` blocks till next event
        if let Some(evt) = tui.next().await {
            match evt {
                Event::Key(k) => {
                    if let KeyCode::Char('q') = k.code {
                        break;
                    }
                }
                _ => {}
            }
        };
    }

    tui.exit()?;

    debug!("finished");

    Ok(())
}

fn ui(frame: &mut Frame, _model: &StartSpringIoModel) {
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
        "Spring Initializr",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
