# ADR-001: Substitute NVI with ONBV in v0.1.0-alpha.0 seed

**Status**: Accepted
**Date**: 2026-05-09
**Deciders**: Victor Gutierrez

## Context

The original `docs/SPEC.md` v0.1 seeded `faith` with **two translations**:

- **KJV** — King James Version (English, public domain)
- **NVI** — Nova Versão Internacional (Portuguese, Biblica)

The intent was to ship one English and one Portuguese translation that fulfil
`faith`'s "agent-first, multi-locale, offline" promise from day one.

During the v0.1.0-alpha.0 implementation we re-checked licensing on the
HelloAOLab `bible-api` catalog — `faith`'s sole upstream data source. The
relevant findings:

| Translation | HelloAO ID  | License            | Redistributable in `faith` |
| ----------- | ----------- | ------------------ | -------------------------- |
| KJV         | `eng_kjv`   | Public Domain      | Yes                        |
| NVI         | *not listed* | Biblica proprietary | **No**                     |
| ONBV        | `por_onbv`  | CC BY-SA 4.0 (Biblica) | **Yes**                    |

NVI (*Nova Versão Internacional*) is **not** in the HelloAO catalog at all,
because Biblica licenses NVI commercially and forbids free redistribution of
the full text. Bundling NVI in `faith install` would either:

1. Require us to host the text ourselves (license violation), or
2. Require a paid Biblica API key (defeats "offline-by-default"), or
3. Ship as an empty stub (broken UX).

None of these are acceptable for v0.1.

ONBV (*Open Nova Bíblia Viva 2007*) is a **separate** Biblica translation that
the publisher explicitly released under Creative Commons Attribution-ShareAlike
4.0 International. It is a complete 66-book Portuguese Bible (31,105 verses),
in the HelloAO catalog as `por_onbv`, and its license permits unmodified
redistribution with attribution. It satisfies every functional role NVI was
playing in the v0.1 seed (modern Portuguese, complete canon, agent-friendly),
without the licensing trap.

## Decision

For v0.1.0-alpha.0, **substitute NVI with ONBV** in the seed catalog:

- The "ready to ship" criterion for v0.1.0-alpha.0 is `faith install KJV ONBV`
  succeeds and `faith get`, `faith batch`, `faith list`, `faith manifest` all
  return well-formed `faith.v1` JSON for both translations.
- Public-facing translation aliases are short uppercase ASCII (`KJV`, `ONBV`)
  per the spec's citation format. The HelloAO IDs (`eng_kjv`, `por_onbv`) are
  internal mapping details exposed only via the manifest's `source_url`.
- The `data::books` table continues to expose `(canonical_id, helloao_id, …)`
  for the 66 USFM book IDs. JSON output uses `canonical_id` (e.g. `JHN`),
  matching the citation format `KJV/JHN/3/16` in `docs/SCHEMA.md`.

NVI is not removed from the long-term roadmap — if Biblica ever publishes NVI
under a redistributable license, `faith install NVI` becomes a one-line
catalog addition. Until then, ONBV is the production seed.

## Consequences

### Positive

- v0.1.0-alpha.0 ships with a fully functional Portuguese translation, on time.
- Zero licensing risk; CC BY-SA 4.0 is explicit and well-understood.
- HelloAO remains `faith`'s sole upstream — no second integration to maintain.
- The decision is reversible: a future `faith install NVI` can coexist with
  `ONBV` once licensing allows.

### Negative

- Users expecting NVI specifically will get ONBV by default. ONBV is less
  well-known than NVI in Brazilian evangelical circles, which may cause minor
  surprise. Mitigated by clear documentation in `README.md`, `CHANGELOG.md`,
  and this ADR.
- `faith` becomes a downstream of CC BY-SA: derivative works of `faith`'s
  bundled ONBV text inherit ShareAlike. The `faith` source code itself remains
  MIT OR Apache-2.0 — only the bundled ONBV verse text carries CC BY-SA, and
  `faith install` records this license in the local `translations` table.

### Neutral

- The CLI surface, schema, citation format, and SQLite layout are **unchanged**
  by this decision. Only the seed catalog and a few documentation strings move.

## References

- HelloAOLab catalog: <https://bible.helloao.org/api/available_translations.json>
- ONBV license metadata: <https://ebible.org/Scriptures/details.php?id=poronbv>
- CC BY-SA 4.0: <https://creativecommons.org/licenses/by-sa/4.0/>
- `docs/SPEC.md` (original v0.1 spec, NVI seed)
- `docs/SCHEMA.md` (canonical `faith.v1` output contract)
