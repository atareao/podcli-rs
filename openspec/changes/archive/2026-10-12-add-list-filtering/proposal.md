# Proposal

## Why

`podcli list` accepts `--first`, `--last`, and `--json`, but ignores all three: `Commands::List` in `src/main.rs` only reads `--url`. The README documents filtering and JSON output that do not exist, and `--json` is declared as `Option<bool>`, so the documented form `podcli list --url … --json` fails to parse with "a value is required for '--json <JSON>'". `list` also prints episodes in arbitrary `HashMap` iteration order.

## What Changes

- `list` honors `--first <N>`: output is limited to the first N episodes in the listing order.
- `list` honors `--last <N>`: output is limited to the last N episodes in the listing order.
- `list` emits a deterministic order: episodes ordered by id descending (newest first), matching `Podcast::get_titles`.
- `--first` and `--last` become `Option<usize>` and reject non-numeric input; supplying both together is a usage error.
- `--json` becomes a valueless boolean flag (`ArgAction::SetTrue`) that prints a single JSON array of episode objects to stdout.
- In `--json` mode, stdout contains ONLY the JSON array: the `get_rss` per-item `println!("{id} - {title}")`, the spinner, and the `=====` headers are suppressed or kept off stdout.
- **BREAKING**: `--json <VALUE>` (which required a value and was unusable) is replaced by the valueless `--json` flag; the README's documented form now works.

## Capabilities

### New Capabilities
- `cli-list`: observable behavior of the `podcli list` subcommand — episode ordering, `--first`/`--last` filtering, and `--json` output.

### Modified Capabilities
<!-- none -->

## Impact

- `src/main.rs`: `ListArgs` contract (types, conflicts) and the `Commands::List` handler.
- `src/podcast.rs`: `get_rss` must stop writing to stdout (diagnostics move to `tracing`); `Episode` gains serialization.
- `Cargo.toml`: add `serde` (derive) and `serde_json`.
- `README.md`: the documented `--first`/`--last`/`--json` examples become accurate.
- No changes to `interactive`.
