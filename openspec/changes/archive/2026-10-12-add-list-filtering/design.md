# Design

## Context

See proposal.md — Why. All `list` logic lives in the `Commands::List` arm of `main()` in `src/main.rs`; `ListArgs` declares `first`/`last` as `Option<String>` and `json` as `Option<bool>` (so it requires a value). `get_rss` in `src/podcast.rs` writes `"{id} - {title}"` to stdout as a side effect while parsing.

## Goals / Non-Goals

Goals:
- Deterministic, testable ordering and filtering for `list`.
- Machine-parseable JSON on stdout with no other stdout noise.

Non-Goals:
- Changing `get_rss` parsing semantics or the `interactive` command.
- Adding further `list` filters (date, search).
- Graceful network error handling; the existing `.unwrap()`s stay.

## Decisions

### Ordering: descending id means newest first
`get_rss` assigns `id = total - index`, so the first `<item>` in the feed gets the highest id. `Podcast::get_titles` already sorts `keys().sorted().rev()`. Reuse that convention: `list` orders by id descending. Alternative considered: ascending id — rejected because it contradicts `get_titles` and the README's "first N" wording.

### Filtering as a pure function
Extract selection into a pure helper, e.g. `fn select<'a>(episodes: &[&'a Episode], first: Option<usize>, last: Option<usize>) -> Vec<&'a Episode>`, so ordering and slicing are unit-testable without network or audio. Alternative: slice inline in the match arm — rejected as untestable.

### --json as a valueless flag, serialization via serde
Change `json: Option<bool>` to `json: bool` with `ArgAction::SetTrue`. Add `serde` (derive) and `serde_json`; derive `Serialize` for `Episode`. Alternative: hand-rolled JSON string building — rejected (escaping bugs, no testability benefit). `serde` is already a transitive dependency of `reqwest`.

### stdout hygiene
Replace the `println!("{id} - {title}")` in `get_rss` with `tracing::debug!`. The spinner already writes to stderr. This keeps `--json` stdout machine-parseable and de-noises `interactive`. Alternative: keep the print and gate it behind the mode — rejected as extra branching for no benefit.

## Risks / Trade-offs

- [Removing the `get_rss` progress lines may surprise users] → they were unstyled debug noise; called out as intentional in the proposal.
- [serde/serde_json add dependencies] → already present transitively via `reqwest`; negligible cost.
- [`--json <VALUE>` now errors] → that form never worked (the value was parsed then ignored); marked BREAKING in the proposal.

## Migration Plan

Single-binary CLI with no persisted state. Ship in the next release; rollback = revert the commit. No data migration.

## Open Questions

None.
