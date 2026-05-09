# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1-alpha.0] - 2026-05-09

### Added

- **`faith info <book> [--tr ID]`** — book metadata (USFM, name, aliases, chapters, verses_total, testament, order)
- **`faith random [--tr ID] [--book USFM] [--scope verse|chapter|ot|nt] [--seed N]`** — deterministic random verse/chapter with FAITH_SEED env support
- **Multi-chapter range parser** — `faith get "john 3:16-4:2"` now parses and fetches cross-chapter ranges; max 500 verses per range (E_RANGE_TOO_LARGE exit 2)
- **`faith diff <ref> --tr ID1,ID2[,ID3...]`** — side-by-side translation comparison; min 2 translations
- **`faith stats [--tr ID]`** — global or per-translation observability (books, chapters, verses, OT/NT split, mtime)
- **`faith completions <shell>`** — shell completion scripts (bash/zsh/fish/powershell/elvish)
- **Tabular output formats** — `--format tsv|csv` on `get`, `batch`, `random`, `diff`, `list` subcommands
  * RFC 4180 CSV with auto-quoting
  * TSV with atomic rows (tab/newline/CR replaced with space in text)
  * Verse-based rows: `translation,book,chapter,verse,text`
- **`faith cache size|clear|path`** — storage management
  * `cache size`: display db/cache/manifest bytes
  * `cache clear --confirm`: idempotent deletion of ~/.faith/cache/
  * `cache path`: print ~/.faith directory

### Changed

- Bumped version to `0.1.1-alpha.0` (dev version)
- README: expanded usage examples to cover all 8 new subcommands
- SCHEMA.md: documented new output types (BookInfo, RandomOut, Range, Diff, Stats, CacheStats)

### Test Coverage

- 39 integration tests (up from 30 in v0.1.0-alpha.0)
  * Deterministic seed reproducibility
  * Range parsing and overflow validation
  * Tabular format escaping (CSV quotes, TSV atomicity)
  * Translation comparison and book scoping
  * Cache dir operations (size, clear, path)

## [0.1.0-alpha.0] - 2026-05-09

### Added

- Initial repository scaffold
- Spec v0.1 (`docs/SPEC.md`)
- Schema v1 contract (`docs/SCHEMA.md`)
- Reference parser design (`docs/REFERENCES.md`)
- Cargo manifest, dual MIT / Apache-2.0 licensing
- CI skeleton, Contributor Covenant CoC

## [0.1.0-alpha.0] - 2026-05-09

First tagged pre-alpha. No published binary yet.

### Added

- `faith get <REF> [--tr <T1[,T2...]>] [--format json|text]` — single / multi-translation lookup
- `faith batch [--tr <T>] [--format json|text]` — JSON-array stdin, ordered output
- `faith list translations [--installed]` and `faith list books --tr <T>`
- `faith install <T1> [<T2>...]` — fetches from HelloAO API into `~/.faith/`
- `faith manifest` — capability + installed catalog snapshot
- Multi-lingual reference parser (PT + EN, ≥30 golden cases each)
- Canonical citation format `{TR}/{USFM_BOOK}/{CHAPTER}[/{VERSE}[-{VERSE}]]`
- SQLite store (FTS5 enabled, `bundled` build)
- 66-book USFM table with HelloAO ID mapping (`canonical_id` ↔ `helloao_id`)
- Stable `faith.v1` JSON schema for verse, range, multi, and error shapes

### Notes

- v0.1.0-alpha.0 ships **KJV** (`eng_kjv`, Public Domain) and **ONBV** (`por_onbv`,
  CC BY-SA 4.0). NVI was originally specified but is not freely redistributable
  via HelloAO; ONBV (Open Nova Bíblia Viva, Biblica) replaces it. See
  [`docs/ADR-001-nvi-substitution.md`](docs/ADR-001-nvi-substitution.md).
- Public-facing translation aliases are uppercase short ASCII (`KJV`, `ONBV`);
  HelloAO IDs (`eng_kjv`, `por_onbv`) are internal and surface only in the
  manifest's `source_url`.
