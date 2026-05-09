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

## Manifest

```json
{
  "schema": "faith.v1",
  "version": "0.1.0",
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
    {"name": "get",      "args": ["ref", "tr?"]},
    {"name": "batch",    "args": ["tr?", "stdin: refs[]"]},
    {"name": "list",     "args": ["kind"]},
    {"name": "install",  "args": ["tr+"]},
    {"name": "manifest", "args": []}
  ]
}
```

Agents should call `faith manifest` once on startup to discover capabilities and installed translations.
