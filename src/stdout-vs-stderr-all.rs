#!/usr/bin/env rust-script

//! ```cargo
//! ratatui = "0.25.0"
//! crossterm = "0.27.0"
//! anyhow = "1.0.76"
//! enum-iterator = "1.4.1"
//! lazy_static = "1.4.0"
//! palette = "0.7.3"
//! rand = "0.8.5"
//! ```

use std::{
    fmt,
    fs::File,
    io::{self, BufWriter, LineWriter, Write},
    os::fd::{FromRawFd, OwnedFd},
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use enum_iterator::{last, next_cycle, Sequence};
use lazy_static::lazy_static;
use palette::{convert::FromColorUnclamped, Hsv, Srgb};
use ratatui::{prelude::*, widgets::*};

lazy_static! {
    static ref RAW_STDOUT_FD: OwnedFd = unsafe { OwnedFd::from_raw_fd(1) };
}

#[derive(Copy, Clone, Debug, Default, Sequence)]
enum IoStream {
    #[default]
    Stdout,
    LineBufferedStdout,
    BlockBufferedStdout,
    Stderr,
    LineBufferedStderr,
    BlockBufferedStderr,
}

impl fmt::Display for IoStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IoStream::Stdout => "stdout (unbuffered)",
                IoStream::LineBufferedStdout => "stdout (line buffered)",
                IoStream::BlockBufferedStdout => "stdout (block buffered)",
                IoStream::Stderr => "stderr (unbuffered)",
                IoStream::LineBufferedStderr => "stderr (line buffered)",
                IoStream::BlockBufferedStderr => "stderr (block buffered)",
            }
        )
    }
}

impl IoStream {
    fn as_stream(&self) -> Result<Box<dyn Write>> {
        Ok(match self {
            IoStream::Stdout => Box::new(File::from(RAW_STDOUT_FD.try_clone()?)),
            IoStream::LineBufferedStdout => Box::new(io::stdout()),
            IoStream::BlockBufferedStdout => Box::new(BufWriter::new(io::stdout())),
            IoStream::Stderr => Box::new(io::stderr()),
            IoStream::LineBufferedStderr => Box::new(LineWriter::new(io::stderr())),
            IoStream::BlockBufferedStderr => Box::new(BufWriter::new(io::stderr())),
        })
    }
}

#[derive(Debug)]
struct Fps {
    frame_count: usize,
    last_instant: Instant,
    fps: Option<f32>,
}

impl Default for Fps {
    fn default() -> Self {
        Self {
            frame_count: 0,
            last_instant: Instant::now(),
            fps: None,
        }
    }
}

impl Fps {
    fn tick(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed();
        // update the fps every second, but only if we've rendered at least 2 frames (to avoid
        // noise in the fps calculation)
        if elapsed > Duration::from_secs(1) && self.frame_count > 2 {
            self.fps = Some(self.frame_count as f32 / elapsed.as_secs_f32());
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }
}

struct FpsWidget<'a> {
    fps: &'a Fps,
}

impl<'a> Widget for FpsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(fps) = self.fps.fps {
            let text = format!("{:.1} fps", fps);
            Paragraph::new(text).render(area, buf);
        }
    }
}

struct RgbColorsWidget<'a> {
    /// the colors to render - should be double the height of the area
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
    title: Paragraph<'a>,
    fps_widget: FpsWidget<'a>,
    rgb_colors_widget: RgbColorsWidget<'a>,
}

impl<'a> AppWidget<'a> {
    fn new(app: &'a App) -> Self {
        let title = Paragraph::new(vec![Line::styled(
            app.current_stream.to_string(),
            Style::new().bold(),
        )])
        .alignment(Alignment::Center);
        Self {
            title,
            fps_widget: FpsWidget { fps: &app.fps },
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
        let title_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(8)])
            .split(main_layout[0]);

        self.title.render(title_layout[0], buf);
        self.fps_widget.render(title_layout[1], buf);
        self.rgb_colors_widget.render(main_layout[1], buf);
    }
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    switch_stream: bool,
    current_stream: IoStream,
    // a 2D vector of the colors to render
    // calculated when the size changes as this is expensive to calculate every frame
    colors: Vec<Vec<Color>>,
    last_size: Rect,
    fps: Fps,
    frame_count: usize,
}

impl App {
    pub fn run(io_stream: IoStream) -> Result<bool> {
        let mut terminal = init_terminal(io_stream.as_stream()?)?;
        let mut app = App {
            current_stream: io_stream,
            ..Default::default()
        };

        while !app.should_quit && !app.switch_stream {
            app.tick();

            terminal.draw(|frame| {
                let size = frame.size();
                app.setup_colors(size);
                frame.render_widget(AppWidget::new(&app), size);
            })?;
            app.handle_events()?;
        }
        restore_terminal(io_stream.as_stream()?)?;
        Ok(app.should_quit)
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        self.fps.tick();
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs_f32(1.0 / 60.0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(' ') {
                    self.switch_stream = true;
                };
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
            // more randomness towards the bottom
            let randomness_factor = (height - y) as f32 / height as f32;
            for _ in 0..width {
                let base_value = randomness_factor * ((height - y) as f32 / height as f32);
                // adjust the range as needed
                let random_offset: f32 = rng.gen_range(-0.1..0.1);
                let value = base_value + random_offset;
                // clamp the value to ensure it stays within the valid range [0.0, 1.0]
                let value = value.max(0.0).min(1.0);
                // set hue to 0 for grayscale
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
    let mut io_stream = last::<IoStream>().unwrap_or_default();
    loop {
        io_stream = next_cycle(&io_stream).unwrap_or_default();
        if App::run(io_stream)? {
            break;
        }
    }
    Ok(())
}
