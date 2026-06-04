# podcli

<p align="center">
  <strong>A fast, terminal-based podcast manager and player written in Rust.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/atareao/podcli-rs/release.yml?branch=main&style=flat-square" alt="build"/>
  <img src="https://img.shields.io/github/last-commit/atareao/podcli-rs?style=flat-square" alt="last-commit"/>
  <img src="https://img.shields.io/github/license/atareao/podcli-rs?style=flat-square" alt="license"/>
  <img src="https://img.shields.io/crates/v/podcli?style=flat-square" alt="crates.io"/>
</p>

---

## Table of Contents

- [Description](#description)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [List Command](#list-command)
  - [Interactive Mode](#interactive-mode)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

---

## Description

`podcli` is a command-line podcast manager written in Rust. It lets you discover, download, and play podcast episodes directly from your terminal. Designed for terminal enthusiasts who want a fast, lightweight, and dependency-free experience.

---

## Features

- **Fast binary** — Single-file release, no runtime needed
- **RSS parsing** — Fetch and parse podcast feeds
- **Audio playback** — Play episodes locally using `rodio`
- **Interactive mode** — Browse and play episodes with an interactive UI
- **JSON output** — Easy integration with scripts and other tools
- **Configurable logging** — Control verbosity via `RUST_LOG`

---

## Installation

### Prerequisites

- **From source:** Rust and Cargo
- **Linux:** `libasound2-dev` and `pkg-config` (required for audio)
- **Docker:** Available as an alternative with no host dependencies

### Install from crates.io

```sh
cargo install podcli
```

### Build from source

```sh
git clone https://github.com/atareao/podcli-rs.git
cd podcli-rs
cargo build --release
# Binary available at target/release/podcli
```

### Using Docker

```sh
docker build -t podcli .
docker run --rm -it podcli
```

---

## Usage

### List Command

List episodes from an RSS feed:

```sh
# List all episodes
podcli list --url https://feeds.simplecast.com/54nVymd2

# List first 5 episodes
podcli list --url https://feeds.simplecast.com/54nVymd2 --first 5

# List last 3 episodes
podcli list --url https://feeds.simplecast.com/54nVymd2 --last 3

# Output as JSON
podcli list --url https://feeds.simplecast.com/54nVymd2 --json
```

### Interactive Mode

Browse episodes with an interactive menu:

```sh
# Basic usage
podcli interactive --url https://feeds.simplecast.com/54nVymd2

# Specify download directory
podcli interactive --url https://feeds.simplecast.com/54nVymd2 --download-dir ~/podcasts

# Change playback speed (0.5 = half, 2.0 = double)
podcli interactive --url https://feeds.simplecast.com/54nVymd2 --speed 1.5
```

Interactive mode options:
1. **List episodes** — Display all episodes (with duration)
2. **Get episode** — Show details for a specific episode
3. **Play episode** — Download and play an episode
4. **Reload** — Refresh the RSS feed
5. **Exit** — Quit the application

### Global Options

Available for all subcommands:

- `--download-dir <DIR>` — Directory to store downloaded episodes (default: `/tmp`)
- `--speed <SPEED>` — Playback speed: `0.5` (half), `1.0` (normal), `2.0` (double)

### Environment Variables

```sh
# Change log level (default: DEBUG)
RUST_LOG=info podcli list --url https://example.com/feed
RUST_LOG=debug podcli interactive --url https://example.com/feed
```

Available levels: `trace`, `debug`, `info`, `warn`, `error`

---

## Development

### Project Structure

```
podcli-rs/
├── Cargo.toml       # Project configuration and dependencies
├── src/
│   ├── main.rs      # CLI entrypoint, commands, playback
│   └── podcast.rs   # RSS parser, Podcast/Episode types
├── Dockerfile       # Multi-arch build
├── Makefile         # Build helpers
└── README.md
```

### Useful Commands

```sh
# Build in debug mode
cargo build

# Run during development
cargo run -- list --url https://feeds.simplecast.com/54nVymd2

# Run tests
cargo test

# Build release
cargo build --release
```

### Key Dependencies

- `clap` — CLI argument parser (derive mode)
- `rodio` — Audio playback
- `roxmltree` — RSS/XML parsing
- `reqwest` — HTTP client
- `tokio` — Async runtime
- `termimad` — Terminal markdown rendering
- `inquire` — Interactive prompts

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/my-feature`)
5. Open a pull request describing your changes

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Contact

For questions or bug reports, please use the [issue tracker](https://github.com/atareao/podcli-rs/issues).
