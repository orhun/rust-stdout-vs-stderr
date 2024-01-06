#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! crossterm = "0.27.0"
//! ratatui = "0.25.0"
//! ```

use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

/// Handle key events.
fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            // Quit when 'q' is pressed.
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Render the widgets.
fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("__QQ\n(_)_\">")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("blog.orhun.dev")
                    .borders(Borders::ALL),
            ),
        frame.size(),
    );
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
