<div align="center">

<img src="assets/logo.svg" alt="faith" width="180" />

# faith

**The Bible. For agents. Universal. Open.**

[![Crates.io](https://img.shields.io/crates/v/faith.svg?style=flat-square&color=10b981)](https://crates.io/crates/faith)
[![Downloads](https://img.shields.io/crates/d/faith.svg?style=flat-square&color=10b981)](https://crates.io/crates/faith)
[![docs.rs](https://img.shields.io/docsrs/faith?style=flat-square&color=10b981)](https://docs.rs/faith)
[![Build](https://img.shields.io/github/actions/workflow/status/V-Gutierrez/faith/ci.yml?style=flat-square&label=build)](https://github.com/V-Gutierrez/faith/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.83-blue?style=flat-square)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](#license)
[![GitHub Stars](https://img.shields.io/github/stars/V-Gutierrez/faith?style=flat-square&color=yellow)](https://github.com/V-Gutierrez/faith)
[![Awesome CLI](https://img.shields.io/badge/awesome--cli-%F0%9F%93%9F-brightgreen?style=flat-square)](https://github.com/alebcay/awesome-cli)

</div>

`faith` is a lightweight, blazing-fast Bible CLI built specifically **for AI agents and automated workflows**. It provides canonical USFM citations, token-efficient JSON outputs, full-text FTS5 search, and is completely offline.

Say goodbye to scraping web pages or parsing unstructured text. Give your agent `faith`, and let it fetch, search, and diff Scripture in milliseconds.

---

## ⚡ Quick Start

```bash
# 1. Install faith
cargo install faith

# 2. Download translations (e.g., King James and Nova Bíblia Viva)
faith install KJV ONBV

# 3. Get structured JSON instantly (offline)
faith get "John 3:16" --tr KJV
# {"schema":"faith.v1","ref":"KJV/JHN/3/16","text":"For God so loved..."}
```

---

## 🤖 For Agents: The `faith` Skill

If you are an AI agent configuring your environment or adding `faith` to your toolkit, here is your system prompt/skill block to understand how to use it:

<details>
<summary><b>Click to expand Agent Skill Prompt</b></summary>

```markdown
You have access to the `faith` CLI, an offline, JSON-first tool for fetching and searching the Bible.
Always use `faith` instead of searching the web for Bible verses.

**Key Commands:**
- `faith manifest` - Call this ONCE at startup. It returns a JSON of all installed tools, translations (`KJV`, `ONBV`, `BLJ`, etc.), and available commands.
- `faith get <ref> [--tr ID] [--lang pt|en]` - Fetch a verse or range (e.g., `faith get "John 3:16-18" --lang en`). Outputs JSON.
- `faith search <query> [--tr ID] [--lang pt|en] [--limit 10]` - Perform a full-text BM25 search. Outputs ranked snippets.
- `faith diff <ref> --tr ID1,ID2` - Compare a verse across multiple translations side-by-side.
- `faith random [--lang pt|en] [--scope ot|nt]` - Get a random verse (useful for daily devotionals or random seeds).
- `faith install <ID1> <ID2>` - If a translation is missing, check `available_translations` in the manifest and install it.

**Rules:**
1. Default output is always JSON (`faith.v1` schema). Do not try to parse it as raw text unless you pass `--format text`.
2. For multiple verses, pass ranges like `"Gen 1:1-5"`. Max limit is 500 verses.
3. You can resolve translations by language directly using `--lang pt` or `--lang en` if you don't know the exact translation ID.
4. Bible book references are extremely flexible (e.g., `1 Coríntios 13`, `Jn 3.16`, `רומיים ח:כח`).
```
</details>

---

## 🛠️ Installation

### macOS / Linux (Homebrew)
The recommended way for macOS and Linux users:
```bash
brew install v-gutierrez/tap/faith
```

### Using Cargo (Rust Package Manager)
The easiest way to install from source:
```bash
cargo install faith
```

### Pre-built Binaries
Download the latest binaries for macOS, Linux, and Windows from the [GitHub Releases](https://github.com/V-Gutierrez/faith/releases).

Extract the binary and place it in your `$PATH`.

---

## 📖 Usage Examples

### Fetching & Reading
```bash
# Single verse (resolves by language automatically)
faith get "João 3:16" --lang pt

# Parallel diff across multiple translations
faith diff "John 3:16" --tr KJV,ONBV,BLJ

# Cross-chapter ranges
faith get "John 3:16-4:2" --tr KJV
```

### Searching
```bash
# Full text search (FTS5 BM25 Ranked)
faith search "shepherd" --lang en --limit 5

# Search specific translation
faith search "amor" --tr ONBV --format text
```

### Discovery & Utilities
```bash
# See what is installed and what commands are available
faith manifest

# Get book metadata (chapters, verses, aliases)
faith info john --tr KJV

# Random verse (supports seed for determinism)
faith random --lang pt --scope nt
```

---

## 📊 Why `faith`?

| Capability                        | `faith`                | Traditional CLI Readers |
| --------------------------------- | ---------------------- | ----------------------- |
| Primary audience                  | **Agents / Scripts**   | Humans (TUI)            |
| JSON output (default)             | ✅                     | ❌                      |
| Full-text FTS5 Search (BM25)      | ✅                     | varies                  |
| Canonical USFM refs (`JHN/3/16`)  | ✅                     | ❌                      |
| Multi-translation parallel diff   | ✅                     | ❌                      |
| Multi-locale ref parser           | ✅                     | ❌                      |
| Deterministic via `--seed`        | ✅                     | ❌                      |
| Cold start                        | **~50 ms** (Rust)      | Slower (Node/Python)    |
| Cross-chapter ranges              | ✅                     | varies                  |

---

## 🌍 Ecosystem

- [📦 Crates.io](https://crates.io/crates/faith) — Published Rust crate
- [📚 Docs.rs](https://docs.rs/faith) — API documentation
- [🍺 Homebrew Tap](https://github.com/V-Gutierrez/homebrew-tap) — macOS / Linux package
- [🤖 Agent Skill](https://github.com/V-Gutierrez/faith?tab=readme-ov-file#-for-agents-the-faith-skill) — System prompt for AI agents

## 📦 Data & Translations

Data is seeded from the [Free Use Bible API](https://bible.helloao.org) (HelloAOLab). The API is completely free, offline-first, and respects individual translation licenses.

Currently available translations (v0.2.0):
- **English:** `KJV`
- **Portuguese:** `ONBV`, `BLJ`, `BSL`, `BLT`, `TFT`

Run `faith manifest` to see the full list of available translations you can install via `faith install <ID>`.

---

## 🤝 Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). TDD is enforced — write a failing test before writing production code.

**Topics:** `cli` `bible` `ai-tools` `json` `rust` `fts5` `search` `homebrew` `macos` `linux`

License: Dual-licensed under MIT or Apache-2.0. Bible texts retain their original licenses.
