# Proposal

## Why

The README documents `--download-dir` and `--speed` as "Global Options — Available for all subcommands", but they are declared on the top-level `Cli` without `global = true`, so the documented form `podcli interactive --url … --download-dir …` fails to parse. Separately, `podcli list --help` shows the inherited placeholder description "Adds files to myapp".

## What Changes

- Mark `--download-dir` and `--speed` as `global`, so they are accepted both before and after the subcommand (matching the README).
- Correct the `list` subcommand description from "Adds files to myapp" to a description of listing episodes.

## Capabilities

### New Capabilities
- `cli-options`: position-independence and help text of the top-level options shared by subcommands.

### Modified Capabilities
<!-- none -->

## Impact

- `src/main.rs`: `Cli` field attributes (`global = true`); `Commands::List` doc comment.
- `README.md`: the documented "Global Options" usage becomes accurate.
- No change to `list` filtering/JSON behavior.
