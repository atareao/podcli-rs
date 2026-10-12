# Tasks

## 1. Characterization baseline (Phase 0)

- [x] 1.1 Add a characterization test asserting `list` currently ignores `--first`/`--last`/`--json` (captures the as-is bug)
- [x] 1.2 Add a characterization test capturing the exact current non-JSON output format (`===== <id> =====` + `Episode::print`)

## 2. RED — failing tests for the new behavior

- [x] 2.1 Unit test `select()`: `--first N` returns the N highest ids, descending
- [x] 2.2 Unit test `select()`: `--last N` returns the N lowest ids, descending
- [x] 2.3 Unit test `select()`: `first` greater than feed size returns all; `first == 0` returns none
- [x] 2.4 clap contract test: `--first abc` is rejected (exit 2)
- [x] 2.5 clap contract test: `--first` + `--last` together is rejected (exit 2)
- [x] 2.6 clap contract test: bare `--json` parses with no value
- [x] 2.7 Test: `--json` output is a single valid JSON array (via `serde_json::from_str`)
- [x] 2.8 Test: JSON objects contain id/title/description/enclosure/link/image/duration
- [x] 2.9 Test: `--json --first 5` yields an array of the 5 newest episodes

## 3. GREEN — minimal implementation

- [x] 3.1 `Cargo.toml`: add `serde` (derive) and `serde_json`
- [x] 3.2 `podcast.rs`: derive `Serialize` for `Episode`
- [x] 3.3 `podcast.rs`: replace the `println!("{id} - {title}")` in `get_rss` with `tracing::debug!`
- [x] 3.4 `main.rs`: `ListArgs` — `first`/`last` as `Option<usize>`, `json` as `bool` with `ArgAction::SetTrue`, `conflicts_with` between first/last
- [x] 3.5 `main.rs`: implement the pure `select()` helper (descending id, first/last slicing)
- [x] 3.6 `main.rs`: wire `--json` to serialize the selected episodes as a single stdout array

## 4. REFACTOR & verify

- [x] 4.1 `cargo fmt --check`
- [x] 4.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 4.3 `cargo test`
- [x] 4.4 Confirm the README `--first`/`--last`/`--json` examples now parse and behave as documented
- [x] 4.5 `openspec validate add-list-filtering`
- [x] 4.6 `openspec archive add-list-filtering` after approval
