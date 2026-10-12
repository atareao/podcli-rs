# cli-list Specification

## Purpose
Defines the observable behavior of the `podcli list` subcommand: how episodes are ordered, how `--first`/`--last` filter the listing, and what `--json` writes to stdout.

## Requirements

### Requirement: Deterministic episode ordering

`podcli list` SHALL emit episodes ordered by episode id in descending order (newest first), identical to the order used by `Podcast::get_titles`. The output order MUST NOT depend on `HashMap` iteration order.

#### Scenario: Unfiltered listing is ordered newest first
- **WHEN** `podcli list --url <feed>` is run against a feed with episodes
- **THEN** episodes are printed in descending id order (highest id first)

### Requirement: --first limits the listing to the first N episodes

`podcli list --first <N>` SHALL print only the first `N` episodes in the listing order, where `N` is a non-negative integer.

#### Scenario: --first smaller than the feed
- **WHEN** `podcli list --url <feed> --first 5` is run on a feed with more than 5 episodes
- **THEN** exactly the 5 episodes with the highest ids are printed

#### Scenario: --first larger than the feed
- **WHEN** `--first` is greater than the number of episodes
- **THEN** all episodes are printed and the command exits 0

#### Scenario: --first 0
- **WHEN** `--first 0` is given
- **THEN** no episode bodies are printed and the command exits 0

#### Scenario: --first rejects non-numeric input
- **WHEN** `--first abc` is given
- **THEN** the command fails with a clap usage error (exit code 2) and prints no episodes

### Requirement: --last limits the listing to the last N episodes

`podcli list --last <N>` SHALL print only the last `N` episodes in the listing order, where `N` is a non-negative integer.

#### Scenario: --last smaller than the feed
- **WHEN** `podcli list --url <feed> --last 3` is run on a feed with more than 3 episodes
- **THEN** exactly the 3 episodes with the lowest ids are printed

#### Scenario: --last larger than the feed
- **WHEN** `--last` is greater than the number of episodes
- **THEN** all episodes are printed and the command exits 0

### Requirement: --first and --last are mutually exclusive

`--first` and `--last` SHALL NOT be combined in the same invocation.

#### Scenario: Both flags given
- **WHEN** `podcli list --url <feed> --first 2 --last 3` is run
- **THEN** the command fails with a clap usage error (exit code 2)

### Requirement: --json emits a JSON array of episodes

`podcli list --json` SHALL print a single JSON array to stdout, where each element is an object with the episode's `id` (number), `title`, `description`, `enclosure`, `link`, `image`, and `duration` (strings). The flag MUST NOT require a value. When `--json` is combined with `--first` or `--last`, the array SHALL contain only the filtered episodes, in the same order.

#### Scenario: JSON output is a single valid array
- **WHEN** `podcli list --url <feed> --json` is run
- **THEN** stdout parses as one JSON array of episode objects and nothing else is written to stdout

#### Scenario: JSON combined with --first
- **WHEN** `podcli list --url <feed> --json --first 5` is run
- **THEN** stdout is a JSON array of the 5 newest episodes

#### Scenario: JSON object field set
- **WHEN** a feed episode with a duration is listed with `--json`
- **THEN** the corresponding object contains a numeric `id` and non-empty `title`, `enclosure`, and `duration` string fields

### Requirement: --json does not pollute stdout

While `--json` is active, `get_rss` SHALL NOT write per-item progress lines to stdout; the spinner and diagnostic output SHALL go to stderr or be disabled. Non-JSON `list` output SHALL keep the existing human-readable `===== <id> =====` format.

#### Scenario: Stdout is machine-parseable
- **WHEN** `podcli list --url <feed> --json` is run
- **THEN** stdout contains no `=====` headers, no `id - title` lines, and no spinner frames
