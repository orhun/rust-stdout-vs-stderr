# `stdout` vs `stderr` ⚔️

✨ Read the blog post: [**https://blog.orhun.dev/stdout-vs-stderr**](https://blog.orhun.dev/stdout-vs-stderr)

![stdout-vs-stderr-detailed](https://github.com/orhun/rust-stdout-vs-stderr/assets/24392180/5971ba46-62b6-4964-8096-532673983058)

This repository contains the following Rust scripts:

- [`stdout-vs-stderr-all.rs`](./src/stdout-vs-stderr-all.rs) (**default**): Rendering comparison for stdout and stderr using unbuffered / line-buffered / block-buffered writes
- [`simple-tui.rs`](./src/simple-tui.rs): A simple TUI application built with [Ratatui](https://ratatui.rs/) & [Crossterm](https://github.com/crossterm-rs/crossterm)
- [`stdout-vs-stderr.rs`](./src/stdout-vs-stderr.rs): Measuring FPS for stdout and stderr
- [`stdout-vs-stderr-profiler.rs`](./src/stdout-vs-stderr.rs): Profiling stdout and stderr
  - Also see [`run-profiler.sh`](./run-profiler.sh)
- [`linewriter.rs`](./src/writer/linewriter.rs): `LineWriter` example
- [`bufwriter.rs`](./src/writer/bufwriter.rs): `BufWriter` example
- [`block-buffered-stdout.rs`](./src/writer/block-buffered-stdout.rs): Stdout with `BufWriter`
- [`raw-stdout-broken`](./src/unbuffered/raw-stdout-broken.rs): Broken example of unbuffered stdout
  - [`raw-stdout-1`](./src/unbuffered/raw-stdout-1.rs): Unbuffered stdout
  - [`raw-stdout-2`](./src/unbuffered/raw-stdout-2.rs): Unbuffered stdout (alternative)
- [`snowfall.rs`](./src/bonus/snowfall.rs): Happy new year!

## License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat&logo=GitHub&labelColor=1D272B&color=819188&logoColor=white)](./LICENSE-APACHE)

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2024, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
