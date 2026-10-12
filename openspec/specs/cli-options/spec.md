# cli-options Specification

## Purpose
Defines the top-level CLI options that apply to all subcommands and the help text shown for the `list` subcommand.

## Requirements

### Requirement: Global options are position-independent

`--download-dir <DIR>` and `--speed <SPEED>` SHALL be accepted both before and after the subcommand.

#### Scenario: Options before the subcommand
- **WHEN** `podcli --download-dir /tmp --speed 1.5 interactive --url <feed>` is run
- **THEN** parsing succeeds and the values are applied

#### Scenario: Options after the subcommand
- **WHEN** `podcli interactive --url <feed> --download-dir /tmp --speed 1.5` is run
- **THEN** parsing succeeds and the values are applied

#### Scenario: Options accepted after list
- **WHEN** `podcli list --url <feed> --download-dir /tmp` is run
- **THEN** parsing succeeds

### Requirement: list help describes the command

`podcli list --help` SHALL describe listing episodes from a feed, not a placeholder.

#### Scenario: Help text describes listing
- **WHEN** `podcli list --help` is run
- **THEN** the description contains "episodes" and does not contain "myapp"
