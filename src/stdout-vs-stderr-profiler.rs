#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! anyhow = "1.0.76"
//! crossterm = "0.27.0"
//! palette = "0.7.3"
//! rand = "0.8.5"
//! ratatui = "0.25.0"
//! ```

use anyhow::Result;
use std::{
    env,
    io::{stderr, stdout, Write},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use palette::{convert::FromColorUnclamped, Hsv, Srgb};
use ratatui::{prelude::*, widgets::*};

#[derive(Copy, Clone, Debug, Default)]
enum IoStream {
    #[default]
    Stdout,
    Stderr,
}

impl From<String> for IoStream {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "stdout" => Self::Stdout,
            "stderr" => Self::Stderr,
            _ => Self::default(),
        }
    }
}

struct RgbColorsWidget<'a> {
    /// The colors to render - should be double the height of the area
    colors: &'a Vec<Vec<Color>>,
    /// the number of elapsed frames that have passed - used to animate the colors
    frame_count: usize,
}

impl Widget for RgbColorsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = self.colors;
        for (xi, x) in (area.left()..area.right()).enumerate() {
            // animate the colors by shifting the x index by the frame number
            let xi = (xi + self.frame_count) % (area.width as usize);
            for (yi, y) in (area.top()..area.bottom()).enumerate() {
                let fg = colors[yi * 2][xi];
                let bg = colors[yi * 2 + 1][xi];
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
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);
        self.rgb_colors_widget.render(main_layout[1], buf);
    }
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    current_stream: IoStream,
    // a 2d vec of the colors to render, calculated when the size changes as this is expensive
    // to calculate every frame
    colors: Vec<Vec<Color>>,
    last_size: Rect,
    frame_count: usize,
}

impl App {
    pub fn run<W>(io_stream: IoStream, stream: W, exit_after: Duration) -> Result<()>
    where
        W: Write,
    {
        let mut terminal = init_terminal(stream)?;
        let mut app = Self::default();
        app.current_stream = io_stream;
        let start_time = Instant::now();
        while !app.should_quit {
            app.tick();
            terminal.draw(|frame| {
                let size = frame.size();
                app.setup_colors(size);
                frame.render_widget(AppWidget::new(&app), size);
            })?;
            app.handle_events()?;
            if start_time.elapsed() >= exit_after {
                break;
            }
        }
        match io_stream {
            IoStream::Stdout => restore_terminal(stdout())?,
            IoStream::Stderr => restore_terminal(stderr())?,
        }

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
        // double the height because each screen row has two rows of half block pixels
        let height = height * 2;
        self.colors.clear();

        use rand::Rng;
        let mut rng = rand::thread_rng();
        for y in 0..height {
            let mut row = Vec::new();
            let randomness_factor = (height - y) as f32 / height as f32; // More randomness towards the bottom
            for _ in 0..width {
                let base_value = randomness_factor * ((height - y) as f32 / height as f32);
                let random_offset: f32 = rng.gen_range(-0.1..0.1); // Adjust the range as needed
                let value = base_value + random_offset;
                // Clamp the value to ensure it stays within the valid range [0.0, 1.0]
                let value = value.max(0.0).min(1.0);
                let color = Hsv::new(0.0, 0.0, value); // Set hue to 0 for grayscale
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
    let stream = env::var("STREAM").map(IoStream::from).unwrap_or_default();
    let duration = env::var("DURATION")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(5));
    match stream {
        IoStream::Stdout => App::run(stream, stdout(), duration)?,
        IoStream::Stderr => App::run(stream, stderr(), duration)?,
    }
    Ok(())
}
