//! Multi-lingual reference parser → canonical USFM.
//!
//! Accepts PT/EN names + abbreviations, separators `:` `.` `,` and whitespace.
//! Output is a `ParsedRef` with USFM `book` and integer chapter/verse.

use unicode_normalization::UnicodeNormalization;

use crate::books;
use crate::error::{FaithError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedRef {
    pub book: String,
    pub chapter: u16,
    pub verse: Option<u16>,
    pub end_chapter: Option<u16>,
    pub end_verse: Option<u16>,
}

pub fn normalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.nfkd() {
        if ch.is_alphanumeric() {
            for low in ch.to_lowercase() {
                if low.is_ascii_alphanumeric() {
                    out.push(low);
                }
            }
        }
    }
    out
}

pub fn parse(input: &str) -> Result<ParsedRef> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(FaithError::RefParse {
            input: input.to_string(),
        });
    }

    let (book_part, numeric_part) =
        split_book_and_numeric(trimmed).ok_or_else(|| FaithError::RefParse {
            input: input.to_string(),
        })?;

    let book_norm = normalize(&book_part);
    let book = books::alias_index()
        .get(&book_norm)
        .copied()
        .ok_or_else(|| FaithError::RefParse {
            input: input.to_string(),
        })?;

    let (chapter, verse, end_chapter, end_verse) =
        parse_numeric(numeric_part).ok_or_else(|| FaithError::RefParse {
            input: input.to_string(),
        })?;

    Ok(ParsedRef {
        book: book.to_string(),
        chapter,
        verse,
        end_chapter,
        end_verse,
    })
}

fn split_book_and_numeric(s: &str) -> Option<(String, &str)> {
    let mut split_at: Option<usize> = None;
    let mut seen_alpha = false;
    for (i, ch) in s.char_indices() {
        if ch.is_alphabetic() {
            seen_alpha = true;
            continue;
        }
        if ch.is_ascii_digit() && seen_alpha {
            split_at = Some(i);
            break;
        }
    }

    let i = split_at?;
    let book = s[..i].trim().trim_end_matches('.').to_string();
    let rest = s[i..].trim();
    if book.is_empty() || rest.is_empty() {
        return None;
    }
    Some((book, rest))
}

type NumericParts = (u16, Option<u16>, Option<u16>, Option<u16>);

fn parse_numeric(s: &str) -> Option<NumericParts> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (head, range_tail) = match s.find('-') {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };

    let (chapter, verse) = parse_chapter_verse(head)?;

    if let Some(tail) = range_tail {
        let tail = tail.trim();
        if tail.is_empty() {
            return None;
        }
        if let Some((ec, ev)) = parse_chapter_verse_pair(tail) {
            Some((chapter, verse, Some(ec), Some(ev)))
        } else {
            let ev: u16 = tail.parse().ok()?;
            Some((chapter, verse, Some(chapter), Some(ev)))
        }
    } else {
        Some((chapter, verse, None, None))
    }
}

fn parse_chapter_verse(s: &str) -> Option<(u16, Option<u16>)> {
    let s = s.trim();
    if let Some((c, v)) = split_on_separator(s) {
        let chapter: u16 = c.trim().parse().ok()?;
        let verse: u16 = v.trim().parse().ok()?;
        Some((chapter, Some(verse)))
    } else {
        let chapter: u16 = s.trim().parse().ok()?;
        Some((chapter, None))
    }
}

fn parse_chapter_verse_pair(s: &str) -> Option<(u16, u16)> {
    if let Some((c, v)) = split_on_separator(s) {
        let chapter: u16 = c.trim().parse().ok()?;
        let verse: u16 = v.trim().parse().ok()?;
        Some((chapter, verse))
    } else {
        None
    }
}

fn split_on_separator(s: &str) -> Option<(&str, &str)> {
    for (i, ch) in s.char_indices() {
        if matches!(ch, ':' | '.' | ',' | '\u{05C3}') {
            let next = i + ch.len_utf8();
            return Some((&s[..i], &s[next..]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> ParsedRef {
        parse(s).unwrap_or_else(|_| panic!("parse failed: {s}"))
    }

    #[test]
    fn english_basics() {
        let r = p("John 3:16");
        assert_eq!(r.book, "JHN");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse, Some(16));
    }

    #[test]
    fn portuguese_basics() {
        let r = p("João 3:16");
        assert_eq!(r.book, "JHN");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse, Some(16));
    }

    #[test]
    fn portuguese_no_accent() {
        let r = p("Joao 3:16");
        assert_eq!(r.book, "JHN");
    }

    #[test]
    fn abbreviations() {
        assert_eq!(p("Jn 3:16").book, "JHN");
        assert_eq!(p("Jo 3:16").book, "JHN");
        assert_eq!(p("Gn 1:1").book, "GEN");
        assert_eq!(p("Sl 23").book, "PSA");
    }

    #[test]
    fn numbered_books() {
        assert_eq!(p("1 Corinthians 13").book, "1CO");
        assert_eq!(p("1Co 13:4").book, "1CO");
        assert_eq!(p("1 Coríntios 13").book, "1CO");
        assert_eq!(p("Primeira Coríntios 13").book, "1CO");
    }

    #[test]
    fn chapter_only() {
        let r = p("Psalms 23");
        assert_eq!(r.book, "PSA");
        assert_eq!(r.chapter, 23);
        assert_eq!(r.verse, None);
    }

    #[test]
    fn verse_range_same_chapter() {
        let r = p("Ps 23:1-6");
        assert_eq!(r.book, "PSA");
        assert_eq!(r.chapter, 23);
        assert_eq!(r.verse, Some(1));
        assert_eq!(r.end_chapter, Some(23));
        assert_eq!(r.end_verse, Some(6));
    }

    #[test]
    fn verse_range_cross_chapter() {
        let r = p("Jn 3:35-4:2");
        assert_eq!(r.book, "JHN");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse, Some(35));
        assert_eq!(r.end_chapter, Some(4));
        assert_eq!(r.end_verse, Some(2));
    }

    #[test]
    fn dot_and_comma_separators() {
        assert_eq!(p("Jn 3.16").verse, Some(16));
        assert_eq!(p("Jn 3,16").verse, Some(16));
    }

    #[test]
    fn rejects_unknown_book() {
        assert!(parse("Florbal 3:16").is_err());
        assert!(parse("").is_err());
        assert!(parse("3:16").is_err());
    }

    #[test]
    fn rejects_garbage_numeric() {
        assert!(parse("John foo:bar").is_err());
        assert!(parse("John 3:16-").is_err());
    }
}
