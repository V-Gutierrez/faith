# Reference parser

Multi-lingual on input, canonical USFM on output.

## Supported locales (v0.1)

| ISO   | Language    | Example                    |
| ----- | ----------- | -------------------------- |
| `en`  | English     | `John 3:16`, `Jn 3:16`     |
| `pt`  | Português   | `João 3:16`, `Jo 3:16`     |
| `es`  | Español     | `Juan 3:16`, `Jn 3:16`     |
| `fr`  | Français    | `Jean 3:16`, `Jn 3,16`     |
| `de`  | Deutsch     | `Johannes 3,16`, `Joh 3:16`|
| `grc` | Koine Greek | `Ἰωάννην 3:16`             |
| `he`  | Hebrew      | `יוחנן ג:טז`               |

## Grammar

```
ref       := book chapter? (sep verse_range)?
chapter   := INTEGER
verse_range := verse ('-' (chapter ':')? verse)?
sep       := ':' | '.' | ',' | ' ' | '׃'
book      := <localized name or abbreviation, prefix-disambiguated>
```

## Disambiguation

- Numeric prefixes for ordered books: `1 John` → `1JN`, `2 Coríntios` → `2CO`
- Localized ordinals: `Primeira Coríntios`, `Premier Corinthiens`, `Erste Korinther`
- Whole-name match wins over abbreviation
- Case-insensitive, accent-insensitive (NFKD normalize)

## Output

```
parse("João 3:16")        → ParsedRef { book: "JHN", chapter: 3, verse: Some(16), end: None }
parse("1 Coríntios 13")   → ParsedRef { book: "1CO", chapter: 13, verse: None, end: None }
parse("Ps 23:1-6")        → ParsedRef { book: "PSA", chapter: 23, verse: Some(1), end: Some((23, 6)) }
parse("Jn 3:35-4:2")      → ParsedRef { book: "JHN", chapter: 3, verse: Some(35), end: Some((4, 2)) }
```

## Round-trip

For every supported locale, `parse(format(x)) == x` must hold for the canonical alias of each book.

## Test corpus

Each locale has 50 golden cases under `tests/golden/refs/<lang>.tsv`:

```
input \t book \t chapter \t verse_start \t verse_end_chapter \t verse_end
```

See `tests/parser_corpus.rs` for the harness.
