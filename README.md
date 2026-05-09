# faith

> Agent-first Bible CLI. Multi-locale. Deterministic. Offline.

[![Crates.io](https://img.shields.io/crates/v/faith.svg)](https://crates.io/crates/faith)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![CI](https://github.com/V-Gutierrez/faith/actions/workflows/ci.yml/badge.svg)](https://github.com/V-Gutierrez/faith/actions)

`faith` is a Bible CLI built **for AI agents**, not humans. It returns canonical, token-efficient JSON; it ships with offline data; it speaks USFM book IDs; it batches; it goes parallel across locales in one call. There is no TUI, no animation, no color-by-default.

If you want a beautiful Bible reader, use [`christ-cli`](https://github.com/whoisyurii/christ-cli) or [`bible`](https://crates.io/crates/bible). If you want your agent to cite Scripture without burning tokens or shelling out in a loop, use `faith`.

## Why

Existing Bible CLIs are human-first: TUIs, themes, pagination, decoration. Agents do not need any of that — they need:

- **Canonical citations** (`KJV/JHN/3/16`, not `John 3:16 (King James Version)`)
- **Bulk** lookups (100 refs in one call, not 100 shells)
- **Multi-locale parallel** (same passage, several translations, one call)
- **Stable schema** (versioned, snapshotable)
- **Pure offline** (zero rate limits, predictable latency)
- **Fast cold start** (Rust, ~50 ms — matters in agent loops)

## Status

Pre-alpha. Spec frozen at v0.1, see [`docs/SPEC.md`](docs/SPEC.md). v0.1 ships `get`, `batch`, `list`, `manifest` against KJV + NVI seed.

## Install

```bash
cargo install faith
```

Or download a release binary from [Releases](https://github.com/V-Gutierrez/faith/releases).

On first run:

```bash
faith install kjv nvi      # downloads selected translations into ~/.faith/
faith manifest             # confirms what is available
```

## Usage

```bash
# Single verse, one translation
faith get "John 3:16" --tr KJV
# → {"schema":"faith.v1","ref":"KJV/JHN/3/16","text":"For God so loved..."}

# Same passage, several translations (parallel, one call)
faith get "John 3:16" --tr KJV,NVI,SBLGNT

# Batch from stdin
echo '["John 3:16","Romans 8:28","Ps 23"]' | faith batch --tr NVI

# Search
faith search "love your enemies" --tr KJV --limit 5

# Inventory
faith list translations
faith list books --tr NVI

# Capability manifest (what an agent should call before anything else)
faith manifest
```

All commands write JSON to stdout by default. Add `--format text` for plain text.

## Output schema (v1)

```json
{
  "schema": "faith.v1",
  "ref": "KJV/JHN/3/16",
  "translation": "KJV",
  "book": "JHN",
  "book_name": {"en": "John", "pt": "João"},
  "chapter": 3,
  "verse": 16,
  "text": "For God so loved the world...",
  "lang": "en",
  "dir": "ltr"
}
```

Citation format: `{TRANSLATION_ID}/{USFM_BOOK_ID}/{CHAPTER}/{VERSE}[-{VERSE}]`. USFM 3-letter book IDs (`GEN`, `JHN`, `1CO`, ...) are the international standard and round-trip safely across locales.

See [`docs/SCHEMA.md`](docs/SCHEMA.md) for the full v1 contract.

## Reference parser

Multi-lingual on input, canonical on output:

| Input             | Canonical    |
| ----------------- | ------------ |
| `John 3:16`       | `JHN/3/16`   |
| `João 3:16`       | `JHN/3/16`   |
| `Jn 3.16`         | `JHN/3/16`   |
| `1 Coríntios 13`  | `1CO/13`     |
| `Ps 23:1-6`       | `PSA/23/1-6` |
| `רומיים ח:כח`      | `ROM/8/28`   |

PT, EN, ES, FR, DE, GR, HE supported in v0.1. See [`docs/REFERENCES.md`](docs/REFERENCES.md).

## Data

Seeded from the [Free Use Bible API](https://bible.helloao.org) (HelloAOLab) — no auth, no usage limits, no copyright restrictions on the API itself; individual translations carry their own licenses, redistributed unmodified. `faith install` fetches per translation on demand.

## Roadmap

- **v0.1** — `get`, `batch`, `list`, `manifest`, KJV + NVI seed
- **v0.2** — FTS5 search, multi-translation
- **v0.3** — `parallel` (multi-locale), `--mcp` server mode
- **v0.4** — Semantic search via local embeddings (`sqlite-vec` + ONNX MiniLM, opt-in)
- **v0.5** — Full HelloAO import path, ~20 curated translations
- **v1.0** — Stable v1 schema, Homebrew formula, signed releases

## Non-goals

- TUI, navigation, themes
- Reading plans, bookmarks, sermon prep
- Audio Bible
- GUI of any kind

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). TDD enforced — failing test before any production code.

## License

Dual-licensed under MIT or Apache-2.0, at your option. Bible texts retain their original licenses; see each translation's `LICENSE` file under `~/.faith/`.

## Acknowledgements

- [HelloAOLab/bible-api](https://github.com/HelloAOLab/bible-api) — data source
- [USFM](https://ubsicap.github.io/usfm/) — book ID standard
- [Beblia Holy-Bible-XML-Format](https://github.com/Beblia/Holy-Bible-XML-Format) — fallback dataset
