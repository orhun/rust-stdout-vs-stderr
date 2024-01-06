# Stdout vs Stderr

This repository contains the following Rust scripts:

- [`simple-tui.rs`](./src/simple-tui.rs): a simple TUI application built with [Ratatui](https://ratatui.rs/) & [Crossterm](https://github.com/crossterm-rs/crossterm)
- [`stdout-vs-stderr.rs`](./src/stdout-vs-stderr.rs): measuring FPS for stdout and stderr
- [`stdout-vs-stderr-profiler.rs`](./src/stdout-vs-stderr.rs): profiling stdout and stderr
  - Also see [`run-profiler.sh`](./run-profiler.sh)
- [`linewriter.rs`](./src/writer/linewriter.rs): `LineWriter` example
- [`bufwriter.rs`](./src/writer/bufwriter.rs): `BufWriter` example
- [`block-buffered-stdout.rs`](./src/writer/block-buffered-stdout.rs): stdout with `BufWriter`
