# faith schema v1

Versioned: `faith.v1`. Breaking changes require a major bump (`faith.v2`). Additive changes (new optional fields) keep the v1 marker.

## Single verse

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

Required: `schema`, `ref`, `translation`, `book`, `chapter`, `verse`, `text`, `lang`, `dir`.
Optional: `book_name` (locale → name map; only locales we have).

## Range

```json
{
  "schema": "faith.v1",
  "ref": "KJV/JHN/3/16-17",
  "translation": "KJV",
  "book": "JHN",
  "lang": "en",
  "dir": "ltr",
  "verses": [
    {"chapter": 3, "verse": 16, "text": "..."},
    {"chapter": 3, "verse": 17, "text": "..."}
  ]
}
```

Cross-chapter range:

```json
{"ref": "KJV/JHN/3/35-4/2", ...}
```

## Whole chapter

```json
{
  "schema": "faith.v1",
  "ref": "KJV/PSA/23",
  "translation": "KJV",
  "book": "PSA",
  "chapter": 23,
  "lang": "en",
  "dir": "ltr",
  "verses": [{"verse": 1, "text": "..."}, ...]
}
```

## Multi-translation

JSON array, one element per translation, in the order given on the command line:

```json
[
  {"schema":"faith.v1","ref":"KJV/JHN/3/16",...},
  {"schema":"faith.v1","ref":"NVI/JHN/3/16",...}
]
```

## Batch

Same shape as multi-translation: a JSON array, results in the same order as the input refs. Failed items are error objects (see below) at their position.

## Error

```json
{
  "schema": "faith.v1",
  "error": {
    "code": "E_REF_PARSE",
    "message": "could not parse reference: 'foo'",
    "input": "foo"
  }
}
```

Stable codes:

| Code                    | Meaning                                    | Exit |
| ----------------------- | ------------------------------------------ | ---- |
| `E_REF_PARSE`           | reference string could not be parsed       | 2    |
| `E_NOT_FOUND`           | reference is valid but verse does not exist | 3    |
| `E_TRANSLATION_MISSING` | requested translation is not installed     | 4    |
| `E_DATA_MISSING`        | local DB missing or unreadable             | 4    |
| `E_IO`                  | filesystem / network failure during install | 5    |
| `E_RANGE_TOO_LARGE`     | range exceeds 500 verse limit              | 2    |
| `E_FORMAT_UNSUPPORTED`  | output format not supported for command    | 2    |

## Book Info

```json
{
  "schema": "faith.v1",
  "kind": "book_info",
  "translation": "KJV",
  "book": {
    "usfm": "JHN",
    "name": "John",
    "book_name": {"en": "John", "pt": "João"},
    "aliases": ["john", "jhn", "jn", "joh", "jo", "joão", "joao"],
    "chapters": 21,
    "verses_total": 879,
    "testament": "NT",
    "order": 43
  }
}
```

`translation` and `verses_total` are present only when `--tr` is given.

## Diff

```json
{
  "schema": "faith.v1",
  "kind": "diff",
  "ref": "John 3:16",
  "translations": [
    {"id": "KJV", "text": "For God so loved..."},
    {"id": "ONBV", "text": "Porque Deus amou..."}
  ]
}
```

For ranges, each entry has `verses: [...]` instead of `text`.

## Stats (global)

```json
{
  "schema": "faith.v1",
  "kind": "stats.global",
  "translations_installed": 2,
  "total_verses": 62204,
  "db_size_bytes": 12345678,
  "cache_size_bytes": 0,
  "manifest_last_updated": "2026-05-09T11:00:00Z"
}
```

## Stats (per-translation)

```json
{
  "schema": "faith.v1",
  "kind": "stats.translation",
  "translation": "KJV",
  "language": "eng",
  "books": 66,
  "chapters": 1189,
  "verses": 31102,
  "ot_verses": 23145,
  "nt_verses": 7957,
  "installed_at": "2026-05-09T11:00:00Z"
}
```

## Cache Stats

```json
{
  "schema": "faith.v1",
  "kind": "cache_stats",
  "db_bytes": 12345678,
  "cache_bytes": 0,
  "manifest_bytes": 1234,
  "data_dir": "/Users/user/.faith"
}
```

## Search

```json
{
  "schema": "faith.v1",
  "kind": "search",
  "query": "loved",
  "translation": "KJV",
  "matches": [
    {
      "ref": "KJV/JHN/3/16",
      "translation": "KJV",
      "book": "JHN",
      "chapter": 3,
      "verse": 16,
      "snippet": "For God so »loved« the world...",
      "rank": -2.35
    }
  ],
  "total": 1
}
```

`translation` is omitted if no filter was passed. `rank` is BM25 relevance (lower is better).

## Message

Generic structured message (used by `cache clear`, `cache path`):

```json
{
  "schema": "faith.v1",
  "kind": "message",
  "message": "Cleared /Users/.../.faith/cache (freed 10.2MB)"
}
```

## Manifest

```json
{
  "schema": "faith.v1",
  "version": "0.1.1",
  "data_dir": "/Users/.../.faith",
  "translations": [
    {
      "id": "KJV",
      "name": "King James Version",
      "english_name": "King James Version",
      "language": "eng",
      "direction": "ltr",
      "books": 66,
      "verses": 31102,
      "license": "Public Domain",
      "source_url": "https://bible.helloao.org/api/KJV/complete.json",
      "installed_at": "2026-05-09T11:00:00Z"
    }
  ],
  "tools": [
    {"name": "get",         "args": ["ref", "tr?"]},
    {"name": "batch",       "args": ["tr?", "stdin: refs[]"]},
    {"name": "list",        "args": ["kind"]},
    {"name": "install",     "args": ["tr+"]},
    {"name": "manifest",    "args": []},
    {"name": "info",        "args": ["book", "tr?"]},
    {"name": "random",      "args": ["tr?", "lang?", "book?", "scope?"]},
    {"name": "diff",        "args": ["ref", "tr+"]},
    {"name": "stats",       "args": ["tr?"]},
    {"name": "completions", "args": ["shell"]},
    {"name": "cache",       "args": ["sub"]}
  ]
}
```

Agents should call `faith manifest` once on startup to discover capabilities and installed translations.
