#!/usr/bin/env rust-script

use std::io::{self, LineWriter, Write};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let mut writer = LineWriter::new(stdout);

    writer.write_all(b"In Rust's domain where choices gleam,")?;
    eprintln!("[waiting for newline]");
    thread::sleep(Duration::from_secs(1));

    // No bytes are written until a newline is encountered
    // (or the internal buffer is filled).
    writer.write_all(b"\n")?;
    eprintln!("\n[writing the rest]");
    thread::sleep(Duration::from_secs(1));

    // Write the rest.
    writer.write_all(
        b"Ratatui's path, a unique stream.
Terminal canvas, colors bright,
Untraveled road, a different light.
That choice, the difference, in code's delight.",
    )?;

    // The last line doesn't end in a newline,
    // so we have to flush or drop the `LineWriter` to finish writing.
    eprintln!("\n[flush or drop to finish writing]");
    thread::sleep(Duration::from_secs(1));
    writer.flush()?;

    Ok(())
}
