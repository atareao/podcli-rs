# Design

## Context

See proposal.md — Why. `Cli` (top level) declares `download_dir: Option<String>` and `speed: f32`; clap only accepts them before the subcommand unless marked global. `Commands` uses `/// Adds files to myapp` on `List`.

## Goals / Non-Goals

Goals:
- Make the documented "global options" usage actually work.
- Accurate `list` help text.

Non-Goals:
- Changing `list` filtering/JSON behavior.
- Adding new options.

## Decisions

### Use clap `global = true`
Marking the two fields `#[arg(global = true, ...)]` makes clap accept them on any subcommand and keeps a single source of truth on `Cli`. Alternative: duplicate the options on each subcommand — rejected (duplication, divergence risk). Alternative: edit only the README to move flags before the subcommand — rejected because the documented intent is "available for all subcommands".

## Risks / Trade-offs

- [`global` args with a default value] → `--speed` keeps `default_value_t = 1.0`, applied uniformly; no behavioral surprise.
- [CLI surface grows slightly] → intended; matches existing README.

## Migration Plan

Single-binary CLI, no persisted state. Ship in the next release.
