# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
