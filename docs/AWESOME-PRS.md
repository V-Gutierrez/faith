# Awesome-list PRs to open for `faith`

This document lists curated "awesome" lists where `faith` belongs and gives
**copy-paste-ready** entries plus the exact insertion point (alphabetical position)
for each PR. Open them one at a time, sign each commit, and link them back here
when merged.

**Canonical one-liner** (use this everywhere unless the list has its own format):

```markdown
- [faith](https://github.com/V-Gutierrez/faith) - Agent-first Bible CLI. Multi-locale, deterministic, offline JSON output for AI agents and MCP servers.
```

> **Style nudges:** most awesome-lists wrap descriptions at ~80 chars and prefer a
> trailing period. A handful (notably `awesome-rust`) require **no trailing period**.
> Always re-read the contributing guide of the target repo before committing.

---

## 1. `punkpeye/awesome-mcp-servers`

- **Repo:** <https://github.com/punkpeye/awesome-mcp-servers>
- **File:** `README.md`
- **Section:** This list is organized by category. `faith` does not have an MCP
  transport yet (planned for v0.3), so the cleanest fit today is a new
  **"📚 Bible & Scripture"** subsection under the existing **Knowledge & Memory**
  or **Reference** group. If maintainers prefer to wait for the MCP transport,
  reframe the PR as "MCP-ready Bible CLI; native MCP transport tracked in
  [#issue]" and link this repo's roadmap.
- **Alphabetical position:** First (and only) entry in the new subsection. If a
  general "Reference" section already exists, slot `faith` between any entries
  starting with `f-…` (e.g. after `everything`, before `g…`).
- **Line to add:**

  ```markdown
  - [faith](https://github.com/V-Gutierrez/faith) 🦀 📇 - Agent-first Bible CLI with canonical USFM citations, multi-locale (PT/EN/ES/FR/DE/GR/HE) reference parser, deterministic JSON, and offline data. MCP transport planned for v0.3.
  ```

  (The 🦀 = Rust, 📇 = stdio — match the legend at the top of the README.)

---

## 2. `agarrharr/awesome-cli-apps`

- **Repo:** <https://github.com/agarrharr/awesome-cli-apps>
- **File:** `readme.md`
- **Section:** `## Religion` (already exists). If absent in the current revision,
  add it under `## Productivity` in alphabetical order between `Reference` and
  `Search`.
- **Alphabetical position:** Between `bible` (if present) and the next entry
  starting with `g…`. Within the Religion section, `faith` slots before `kjv-cli`
  and after any entry starting with `e…`.
- **Line to add:**

  ```markdown
  - [faith](https://github.com/V-Gutierrez/faith) - Agent-first Bible CLI with USFM citations, multi-locale reference parser, and deterministic JSON output.
  ```

---

## 3. `rust-unofficial/awesome-rust`

- **Repo:** <https://github.com/rust-unofficial/awesome-rust>
- **File:** `README.md`
- **Section:** `## Applications` → `### Utilities` (CLI tools without a more
  specific home land here). If a `### Religion` or `### Bible` subsection ever
  exists, prefer that.
- **Alphabetical position:** Within Utilities, slot between any entry starting
  with `e…` and `g…`. Specifically: after `eva` (if present), before `gitui`.
- **Line to add (NO trailing period — house style):**

  ```markdown
  - [V-Gutierrez/faith](https://github.com/V-Gutierrez/faith) — Agent-first Bible CLI. Canonical USFM citations, multi-locale parser, deterministic JSON, offline by default
  ```

  Note the em dash (—) and `owner/repo` link text — both required by
  awesome-rust's contributing guide.

---

## 4. `sindresorhus/awesome` (root meta-list) — propose new awesome list

- **Repo:** <https://github.com/sindresorhus/awesome>
- **Strategy:** There is no `awesome-bible` list under sindresorhus's umbrella
  (verified via the master index). Two viable paths:
  1. **Create `V-Gutierrez/awesome-bible-tools`** (CLIs, APIs, MCP servers,
     parsers, datasets), seed it with 20+ curated entries, follow
     [the awesome list checklist](https://github.com/sindresorhus/awesome/blob/main/awesome.md),
     wait the required 30 days of activity, then open the PR to add it to the
     master list.
  2. **Submit `faith` to the existing
     [`willitconnect/awesome-christianity`](https://github.com/willitconnect/awesome-christianity)**
     list (or equivalent community-maintained list) under a `## CLI Tools`
     section as an interim move.
- **If pursuing path (1), suggested entry on the master list:**

  ```markdown
  - [Bible Tools](https://github.com/V-Gutierrez/awesome-bible-tools#readme) - CLIs, APIs, MCP servers, and datasets for working with biblical text.
  ```

---

## 5. `wong2/awesome-mcp-servers` (alternative MCP list)

- **Repo:** <https://github.com/wong2/awesome-mcp-servers>
- **File:** `README.md`
- **Section:** Same reasoning as `punkpeye` — open the PR once `faith --mcp`
  ships in v0.3, or open it now under a "Coming soon / MCP-ready CLIs" note if
  the maintainer accepts pre-MCP entries.
- **Line to add:** same as `punkpeye/awesome-mcp-servers` above.

---

## PR opening checklist (per repo)

For each PR Victor opens:

- [ ] Fork target repo, create branch `add-faith`
- [ ] Insert the line above at the **exact alphabetical position** noted
- [ ] Run any linter the repo provides (`awesome-lint`, `markdownlint`, etc.)
- [ ] Open PR with title: `Add faith` (matches house style on most lists)
- [ ] PR body: 2-3 sentences describing `faith`, link to crates.io once published,
      link to the demo asciinema once recorded
- [ ] Sign-off / DCO if the repo requires it
- [ ] Track merge status here (add a ✅ + PR link next to each entry above)

---

## Tracking table

| List                              | PR URL                              | Status   |
| --------------------------------- | ----------------------------------- | -------- |
| punkpeye/awesome-mcp-servers      | _(open me)_                         | pending  |
| agarrharr/awesome-cli-apps        | _(open me)_                         | pending  |
| rust-unofficial/awesome-rust      | _(open me)_                         | pending  |
| sindresorhus/awesome (meta)       | _(needs awesome-bible-tools first)_ | deferred |
| wong2/awesome-mcp-servers         | _(after v0.3 MCP ships)_            | deferred |
