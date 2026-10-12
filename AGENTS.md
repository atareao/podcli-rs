# AGENTS.md

`podcli` — a terminal podcast manager and player. Single Rust binary (`src/main.rs` + `src/podcast.rs`), no workspace.

## Build & run

```sh
cargo build --release   # -> target/release/podcli
cargo run -- <args>     # dev build
```

Linux builds/tests need ALSA headers: `sudo apt-get install libasound2-dev pkg-config` (required by `rodio`). All CI workflows install these.

## Verify (CI gate, in order)

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build
cargo test
```

`.github/workflows/ci.yml` runs exactly this on PRs to `main`/`development` and pushes to `development`. Any clippy warning fails CI.

## Testing

- Unit tests live in `src/main.rs` under `#[cfg(test)] mod tests` (ordering/filtering helper, clap contract via `Cli::try_parse_from`, JSON shape).
- Integration tests live in `tests/list_cli.rs`: they serve a fixture RSS from a local `TcpListener` and drive the real binary via `env!("CARGO_BIN_EXE_podcli")`. No network or audio needed.
- Focus a run with `cargo test <name>` (e.g. `cargo test select_`) or a suite with `cargo test --test list_cli`.

## CLI quirks that will trip you up

- `--download-dir` and `--speed` are `global` top-level options: accepted before or after the subcommand. (`-d` is `--debug`; `--download-dir` has no short form.)
- `list`: `--first <N>` and `--last <N>` are mutually exclusive and limit the listing; episodes are always ordered by id descending (newest first). `--json` prints a single JSON array and nothing else.
- **stdout is reserved for program output.** The `tracing` subscriber is wired to stderr in `main()`, and `get_rss` must not print to stdout — keep it that way so `list --json` stays machine-parseable.
- `interactive` loops forever until Exit or `process::exit(0)`. Playback blocks on `player.sleep_until_end()` — no async streaming despite the `tokio` runtime.
- `RUST_LOG` defaults to `DEBUG` (hardcoded in `main()`); logging uses `tracing`/`tracing-subscriber`, not the `log` crate.
- Network and XML failures panic: `get_rss` and `Episode::download` use `.unwrap()` throughout (`src/podcast.rs`).

## Spec-driven changes (OpenSpec)

`openspec/` holds the specs; schema is `spec-driven`.
- Create a change with `openspec new change <name>` — note `openspec change <name>` is NOT the create command in v1.x.
- Artifacts go in `openspec/changes/<name>/`: `proposal.md`, `specs/<capability>/spec.md`, `design.md`, `tasks.md`. Scenarios MUST use exactly four `####` and `WHEN`/`THEN`.
- Gate with `openspec validate <name>`; once the work is done, `openspec archive <name>` merges the delta into `openspec/specs/`.
- Do not write feature code before its change proposal exists.

## Versioning & release (automated — don't do it by hand)

- Version source of truth is `.vampus.yml` (`current_version`); `vampus` keeps `Cargo.toml`'s `version` in sync.
- Push to `main` triggers `.github/workflows/release-prepare.yml`: it infers the bump (major/minor/patch from Conventional Commits), regenerates `CHANGELOG.md` via `git-cliff` (`cliff.toml`), commits `chore: release vX`, tags `vX`, pushes, and fast-forwards `development`.
- Tag `v*` triggers `.github/workflows/release.yml`: builds Linux x86_64 + aarch64, creates the GitHub release, and publishes to crates.io.
- Do **not** run `just release` or create tags manually — that duplicates the automated bump/tag and risks a double release. `just` recipes: `just list`, `just install`, `just upgrade`, `just release`.
- Use Conventional Commits; `git-cliff` filters non-conventional commits out of the changelog.

## Docker

`Dockerfile` cross-compiles musl via `platform.sh` + `cross`, final stage on `alpine`. The image runs as an unprivileged `dockeruser` (created in the image) with `tini` as PID 1.
