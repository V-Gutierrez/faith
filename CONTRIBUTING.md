# Contributing

Thank you for considering a contribution to `faith`. This project has a tight scope (agent-first Bible CLI) and a strict process. Please read this before opening a PR.

## Scope check

Before any code: does the change fit the [non-goals](README.md#non-goals)? If it adds TUI, themes, bookmarks, audio, or anything human-decorative, the answer is no — please open an issue first to discuss.

## TDD is mandatory

> NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST.

Workflow:

1. Write a failing test in `tests/` or under `#[cfg(test)]`.
2. Run `cargo test`, watch it fail (Red).
3. Write the minimum production code to pass (Green).
4. Refactor with the test still green.
5. Commit Red and Green separately, or in one commit with both clearly visible in the diff.

## Setup

```bash
git clone https://github.com/V-Gutierrez/faith
cd faith
cargo build
cargo test
```

MSRV: 1.83 (stable). Format with `cargo fmt`. Lint with `cargo clippy --all-targets -- -D warnings`. Both run in CI.

## Schema stability

`faith.v1` JSON is a public contract. Any change to its shape requires:

- An updated snapshot test under `tests/snapshots/`
- A `CHANGELOG.md` entry under the next version
- A migration note in `docs/SCHEMA.md` if breaking

Breaking changes bump the schema major (`faith.v2`) and the crate major.

## Reference parser changes

Adding a locale or adjusting an existing one requires:

- A new or updated golden corpus under `tests/golden/refs/<lang>.tsv` (≥ 50 cases)
- All existing locales still passing
- Round-trip property test green

## Commit style

Conventional Commits:

```
feat(parser): add Spanish locale
fix(store): handle empty FTS5 result set
docs(spec): clarify cross-chapter range format
test(cli): snapshot for batch error positioning
chore(ci): pin Rust toolchain to 1.83
```

## Pull requests

- One topic per PR
- CI must be green
- Include `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` output if anything is unusual
- Update `CHANGELOG.md` under `## [Unreleased]`

## Releases

Maintainer-only. Tag `vX.Y.Z`, push, and the release workflow publishes to crates.io and attaches binaries.

## Code of Conduct

See [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Be excellent to each other.

## License

By submitting a PR you agree to license your contribution under MIT OR Apache-2.0, matching the project.
