# Tasks

## 1. RED — failing contract tests

- [x] 1.1 clap test: `podcli interactive --url x --download-dir /tmp` parses Ok
- [x] 1.2 clap test: `podcli interactive --url x --speed 1.5` parses Ok
- [x] 1.3 clap test: `podcli list --url x --download-dir /tmp` parses Ok
- [x] 1.4 test: `podcli list --help` text contains "episodes" and not "myapp"

## 2. GREEN — minimal implementation

- [x] 2.1 `main.rs`: add `global = true` to `download_dir` and `speed` `#[arg(...)]`
- [x] 2.2 `main.rs`: fix the `Commands::List` doc comment

## 3. REFACTOR & verify

- [x] 3.1 `cargo fmt --check`
- [x] 3.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 3.3 `cargo test`
- [x] 3.4 `openspec validate make-cli-options-global`
- [ ] 3.5 `openspec archive make-cli-options-global` after approval
