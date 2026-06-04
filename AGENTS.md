## Build & Run

```sh
cargo build --release          # release binary at target/release/podcli
cargo run -- <subcommand>     # development mode
```

Linux requires `libasound2-dev pkg-config` for the `rodio` audio dependency. CI installs these in `.github/workflows/release.yml`.

## Testing

```sh
cargo test
```

The existing test (`test_get_rss` in `src/main.rs`) is a placeholder that intentionally fails. No meaningful tests exist yet.

## Logging

`RUST_LOG` defaults to `DEBUG` (hardcoded in `src/main.rs:70`). Override with:
```sh
RUST_LOG=info cargo run -- <subcommand>
```

## Architecture

Single binary (`src/main.rs` + `src/podcast.rs`). Not a monorepo.

- `main.rs` — CLI entrypoint (clap derive), interactive loop, audio playback (`rodio`)
- `podcast.rs` — RSS parsing (`roxmltree`), `Podcast`/`Episode` types, download logic

Audio playback blocks on `sink.sleep_until_end()` (no async streaming).

## CI

`.github/workflows/release.yml` triggers on GitHub release only. It builds a Linux binary and uploads assets. It does not run tests.

## Docker

```sh
docker build -t podcli .
docker run --rm -it podcli
```

Multi-arch build via `platform.sh` and `cross` in `Dockerfile`.
