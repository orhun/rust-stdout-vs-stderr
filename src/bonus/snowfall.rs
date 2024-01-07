#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! ratatui = "0.25.0"
//! crossterm = "0.27.0"
//! anyhow = "1.0.76"
//! palette = "0.7.3"
//! rand = "0.8.5"
//! ```

use std::{
    io::{stdout, Write},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use palette::{convert::FromColorUnclamped, Hsv, Srgb};
use rand::{rngs::ThreadRng, Rng};
use ratatui::{prelude::*, widgets::*};

struct RgbColorsWidget<'a> {
    colors: &'a Vec<Vec<Color>>,
    frame_count: usize,
}

impl Widget for RgbColorsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = self.colors;
        for y in (area.top()..area.bottom()).rev() {
            let yi = ((area.bottom() - y - 1) + self.frame_count as u16) % (area.height);
            for (xi, x) in (area.left()..area.right()).enumerate() {
                let fg = colors[yi as usize * 2][xi];
                let bg = colors[yi as usize * 2 + 1][xi];
                buf.get_mut(x, y).set_char('▀').set_fg(fg).set_bg(bg);
            }
        }
    }
}

struct AppWidget<'a> {
    rgb_colors_widget: RgbColorsWidget<'a>,
}

impl<'a> AppWidget<'a> {
    fn new(app: &'a App) -> Self {
        Self {
            rgb_colors_widget: RgbColorsWidget {
                colors: &app.colors,
                frame_count: app.frame_count,
            },
        }
    }
}

impl Widget for AppWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.rgb_colors_widget.render(area, buf);
    }
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    colors: Vec<Vec<Color>>,
    rng: ThreadRng,
    last_size: Rect,
    frame_count: usize,
}

impl App {
    pub fn run() -> Result<()> {
        let mut terminal = init_terminal(stdout())?;
        let mut app = Self::default();

        while !app.should_quit {
            app.tick();
            terminal.draw(|frame| {
                let size = frame.size();
                app.setup_colors(size);
                frame.render_widget(AppWidget::new(&app), size);
            })?;
            app.handle_events()?;
        }
        restore_terminal(stdout())?;
        Ok(())
    }

    fn tick(&mut self) {
        self.frame_count += 1;
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs_f32(1.0 / 60.0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                }
            }
        }
        Ok(())
    }

    fn setup_colors(&mut self, size: Rect) {
        // only update the colors if the size has changed since the last time we rendered
        if self.last_size.width == size.width && self.last_size.height == size.height {
            return;
        }
        self.last_size = size;
        let Rect { width, height, .. } = size;
        let height = height * 2;
        self.colors.clear();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                let base_value = 1.;
                let random_offset: f32 = self.rng.gen_range(-100.0..0.1);
                let value = base_value + random_offset;
                let value = value.max(0.0).min(1.0);
                let color = Hsv::new(0.0, 0.0, value);
                let color = Srgb::<f32>::from_color_unclamped(color);
                let color: Srgb<u8> = color.into_format();
                let color = Color::Rgb(color.red, color.green, color.blue);
                row.push(color);
            }
            self.colors.push(row);
        }
    }
}

fn init_terminal<W>(mut stream: W) -> Result<Terminal<CrosstermBackend<W>>>
where
    W: Write,
{
    enable_raw_mode()?;
    stream.execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stream))?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn restore_terminal<W>(mut stream: W) -> Result<()>
where
    W: Write,
{
    disable_raw_mode()?;
    stream.execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<()> {
    App::run()
}
