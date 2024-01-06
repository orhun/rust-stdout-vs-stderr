# Stdout vs Stderr

This repository contains the following Rust scripts:

- [`simple-tui.rs`](./src/simple-tui.rs): a simple TUI application built with [Ratatui](https://ratatui.rs/) & [Crossterm](https://github.com/crossterm-rs/crossterm)
- [`stdout-vs-stderr.rs`](./src/stdout-vs-stderr.rs): measuring FPS for stdout and stderr
- [`stdout-vs-stderr-profiler.rs`](./src/stdout-vs-stderr.rs): profiling stdout and stderr
  - Also see [`run-profiler.sh`](./run-profiler.sh)
- [`linewriter.rs`](./src/writer/linewriter.rs): `LineWriter` example
- [`bufwriter.rs`](./src/writer/bufwriter.rs): `BufWriter` example
- [`block-buffered-stdout.rs`](./src/writer/block-buffered-stdout.rs): stdout with `BufWriter`
- [`raw-fd-exploit.rs`](./src/bonus/raw-fd-exploit.rs): Exploit described in [this discussion](https://users.rust-lang.org/t/why-is-fromrawfd-unsafe/39670).
- [`raw-stdout-broken`](./src/unbuffered/raw-stdout-broken.rs): Broken example of unbuffered stdout
  - [`raw-stdout-1`](./src/unbuffered/raw-stdout-1.rs): Unbuffered stdout
  - [`raw-stdout-2`](./src/unbuffered/raw-stdout-2.rs): Unbuffered stdout (alternative)
