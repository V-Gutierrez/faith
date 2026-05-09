# faith — Specification v0.1

Format: G/C/I/V/T/B (Goals / Constraints / Implementation / Validation / Tests / Backlog).

## G — Goals

1. **Agent-first CLI** for Bible lookup, search, and citation
2. **Multi-locale by design**, not bolt-on
3. **Deterministic, batch-capable, offline-by-default**
4. **MCP-ready** — clean tool surface for stdin/stdout JSON-RPC mode (v0.3)
5. **Single binary, ~50 ms cold start**

## NG — Non-Goals

- TUI / interactive navigation
- Reading plans, bookmarks, sermon prep
- Audio Bible
- GUI

## C — Constraints

- **Language:** Rust (cold start, distribution, FFI for `sqlite-vec` later)
- **Storage:** SQLite (`bundled`), FTS5 enabled, optional `sqlite-vec`
- **Citation:** USFM 3-letter book IDs (`GEN`, `JHN`, `1CO`)
- **Schema:** versioned (`faith.v1`), JSON canonical, byte-stable
- **Distribution:** crates.io, GitHub releases, Homebrew (v1.0)
- **License:** MIT OR Apache-2.0; data licenses preserved
- **Network:** only on `faith install` and `faith refresh`; lookup/search are pure offline
- **Output:** JSON to stdout by default; `--format text` for humans
- **Exit codes:** `0` ok, `2` parse error, `3` not found, `4` data missing, `5` IO

## Personas

- **Primary:** AI agents (MCP clients, OpenClaw, Copilot, scripted Consi crons)
- **Secondary:** scripts (cron generating devotionals, fact-check pipelines)
- **Tertiary:** power-user humans via shell

## Architecture

```
faith/
├── src/
│   ├── main.rs          # clap entry
│   ├── lib.rs           # re-exports
│   ├── cli/             # subcommands
│   │   ├── get.rs
│   │   ├── batch.rs
│   │   ├── search.rs
│   │   ├── list.rs
│   │   ├── install.rs
│   │   └── manifest.rs
│   ├── core/
│   │   ├── reference.rs # multi-lingual parser → USFM
│   │   ├── citation.rs  # canonical formatter / parser
│   │   ├── schema.rs    # serde types for faith.v1
│   │   └── store.rs     # SQLite access (FTS5)
│   ├── data/
│   │   ├── books.rs     # USFM table + per-locale aliases
│   │   └── installer.rs # HelloAO API client
│   └── error.rs
├── tests/               # integration, snapshot
├── data/                # seed fixtures (KJV verse subset for tests)
├── docs/
│   ├── SPEC.md
│   ├── SCHEMA.md
│   ├── REFERENCES.md
│   └── MCP.md           # v0.3
└── .github/workflows/ci.yml
```

## CLI Surface (v0.1)

```
faith get <REF> [--tr <T1[,T2...]>] [--format json|text]
faith batch [--tr <T>] [--format json|text]                 # reads JSON array of refs from stdin
faith list translations [--lang <iso>] [--installed]
faith list books --tr <T>
faith install <T1> [<T2>...]
faith manifest                                              # capabilities + installed catalog
```

v0.2 adds `search`. v0.3 adds `parallel` and `--mcp` server mode.

## Schema v1 (canonical)

Single verse:

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

Range (chapter or verse range) returns `{"schema":"faith.v1","ref":"KJV/JHN/3/16-17","verses":[ ... ]}`.

Multi-translation returns a JSON array of single-verse / range objects in the order requested.

Errors:

```json
{"schema":"faith.v1","error":{"code":"E_REF_PARSE","message":"could not parse reference: 'foo'"}}
```

Stable error codes: `E_REF_PARSE`, `E_NOT_FOUND`, `E_TRANSLATION_MISSING`, `E_DATA_MISSING`, `E_IO`.

## Citation Format

`{TRANSLATION_ID}/{USFM_BOOK_ID}/{CHAPTER}[/{VERSE}[-{VERSE}]]`

- Translation IDs: uppercase, ASCII (`KJV`, `NVI`, `SBLGNT`)
- USFM book IDs: 3-letter uppercase (`GEN`, `JHN`, `1CO`, `REV`)
- Chapter and verse: positive integers
- Range: same chapter `JHN/3/16-17`; cross-chapter `JHN/3/16-4/2`

Round-trips safely:

```
parse("João 3:16")    → KJV/JHN/3/16
format(KJV, JHN, 3, 16) → "KJV/JHN/3/16"
```

## Reference Parser

Locales in v0.1: PT, EN, ES, FR, DE, GR, HE. Each locale ships a static alias table mapping localized book names + abbreviations to USFM IDs. Parser is locale-agnostic on input — tries all tables, locks on first unambiguous match.

Separator tolerance: `:`, `.`, `,`, ` ` between chapter and verse. Hebrew accepts the literal colon `׃` and standard `:`.

Disambiguation: numeric prefixes (`1`, `2`, `3`) for `1CO`, `2CO`, `1JN` etc.; localized ordinals (`Primeira Coríntios`, `Premier Corinthiens`) supported.

## Data

Source: [Free Use Bible API](https://bible.helloao.org) (HelloAOLab). Per translation:

1. `faith install KJV` → fetches `/api/KJV/complete.json` from `bible.helloao.org`
2. Imports verses into SQLite `~/.faith/bible.db`
3. Records license + source URL in `translations` table

Storage targets:

- `~/.faith/bible.db` — main DB
- `~/.faith/cache/` — raw downloads, `<TRANS>.json`
- `~/.faith/manifest.json` — installed catalog snapshot

## SQLite schema

```sql
CREATE TABLE translations (
  id TEXT PRIMARY KEY,         -- 'KJV'
  name TEXT NOT NULL,          -- 'King James Version'
  english_name TEXT,
  language TEXT NOT NULL,      -- ISO 639-3 'eng'
  direction TEXT NOT NULL,     -- 'ltr' | 'rtl'
  license TEXT,
  source_url TEXT,
  installed_at TEXT NOT NULL
);

CREATE TABLE verses (
  translation TEXT NOT NULL,
  book TEXT NOT NULL,          -- USFM 'JHN'
  chapter INTEGER NOT NULL,
  verse INTEGER NOT NULL,
  text TEXT NOT NULL,
  PRIMARY KEY (translation, book, chapter, verse),
  FOREIGN KEY (translation) REFERENCES translations(id)
);

CREATE VIRTUAL TABLE verses_fts USING fts5(
  text,
  content='verses',
  tokenize='unicode61'
);
```

## V — Validation

- **Determinism:** identical input → byte-identical JSON output
- **Schema stability:** snapshot tests gate breaking changes; `faith.v1` only mutates with major version bump
- **Reference parser correctness:** ≥ 50 cases per locale, golden file
- **Round-trip:** for every supported locale, `parse(format(x)) == x`
- **Performance:**
  - cold start: ≤ 50 ms (release build)
  - single lookup: ≤ 10 ms
  - batch 100 refs: ≤ 200 ms
  - search 1 query / 1 translation: ≤ 100 ms
  - measured via `criterion` benches (v0.2)

## T — Tests (TDD)

Required RED tests before any v0.1 production code:

1. `core::citation` — `parse`, `format`, round-trip table
2. `core::reference` — per-locale parser, 50 cases each
3. `core::store` — open, install translation, lookup, range
4. `cli::get` — single ref, multi-tr, format json|text, error paths
5. `cli::batch` — stdin JSON array, ordering preserved
6. `cli::manifest` — schema snapshot
7. `schema::v1` — snapshot for every output shape (single, range, multi, error)

## B — Backlog

- **v0.1** — `get`, `batch`, `list`, `install`, `manifest`; KJV + NVI seed
- **v0.2** — `search` (FTS5), multi-tr search, `--limit`, criterion benches
- **v0.3** — `parallel` (multi-locale ergonomics), `--mcp` JSON-RPC stdin/stdout, MCP tool surface documented
- **v0.4** — Semantic search: `sqlite-vec` + ONNX MiniLM via `ort`, opt-in, `--semantic` flag
- **v0.5** — Full HelloAO import pipeline; curated 20 translations; signed release artifacts
- **v1.0** — Stable schema v1, Homebrew formula, MSRV pinned, security audit, `cargo-deny`

## Out of scope (forever)

- TUI / colored / paginated output
- Bookmarks, history, plans
- Audio
- Network at lookup time
