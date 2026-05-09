//! Canonical citation parser and formatter.
//!
//! Format: `{TR}/{USFM_BOOK}/{CHAPTER}[/{VERSE}[-{VERSE}]]` with optional
//! cross-chapter range form `{TR}/{BOOK}/{C1}/{V1}-{C2}/{V2}`.

use crate::error::{FaithError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Citation {
    pub translation: String,
    pub book: String,
    pub chapter: u16,
    pub verse: Option<u16>,
    pub end: Option<(u16, u16)>,
}

impl Citation {
    pub fn format(&self) -> String {
        let base = format!("{}/{}/{}", self.translation, self.book, self.chapter);
        match (self.verse, self.end) {
            (None, _) => base,
            (Some(v), None) => format!("{base}/{v}"),
            (Some(v), Some((ec, ev))) if ec == self.chapter => format!("{base}/{v}-{ev}"),
            (Some(v), Some((ec, ev))) => format!("{base}/{v}-{ec}/{ev}"),
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        let mk_err = || FaithError::RefParse {
            input: s.to_string(),
        };
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() < 3 {
            return Err(mk_err());
        }
        let translation = parts[0].trim().to_ascii_uppercase();
        let book = parts[1].trim().to_ascii_uppercase();
        let chapter: u16 = parts[2].parse().map_err(|_| mk_err())?;
        if translation.is_empty() || book.len() != 3 {
            return Err(mk_err());
        }

        match parts.len() {
            3 => Ok(Self {
                translation,
                book,
                chapter,
                verse: None,
                end: None,
            }),
            4 => {
                let verse_part = parts[3];
                if let Some((start, end)) = verse_part.split_once('-') {
                    let v: u16 = start.parse().map_err(|_| mk_err())?;
                    let ev: u16 = end.parse().map_err(|_| mk_err())?;
                    Ok(Self {
                        translation,
                        book,
                        chapter,
                        verse: Some(v),
                        end: Some((chapter, ev)),
                    })
                } else {
                    let v: u16 = verse_part.parse().map_err(|_| mk_err())?;
                    Ok(Self {
                        translation,
                        book,
                        chapter,
                        verse: Some(v),
                        end: None,
                    })
                }
            }
            5 => {
                let (start, ev_str) = parts[3].split_once('-').ok_or_else(mk_err)?;
                let v: u16 = start.parse().map_err(|_| mk_err())?;
                let ec: u16 = ev_str.parse().map_err(|_| mk_err())?;
                let ev: u16 = parts[4].parse().map_err(|_| mk_err())?;
                Ok(Self {
                    translation,
                    book,
                    chapter,
                    verse: Some(v),
                    end: Some((ec, ev)),
                })
            }
            _ => Err(mk_err()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(s: &str) {
        let c = Citation::parse(s).unwrap_or_else(|e| panic!("parse {s}: {e}"));
        assert_eq!(c.format(), s);
    }

    #[test]
    fn parse_chapter_only() {
        let c = Citation::parse("KJV/JHN/3").unwrap();
        assert_eq!(c.chapter, 3);
        assert!(c.verse.is_none());
        assert_eq!(c.format(), "KJV/JHN/3");
    }

    #[test]
    fn parse_single_verse() {
        let c = Citation::parse("KJV/JHN/3/16").unwrap();
        assert_eq!(c.verse, Some(16));
        assert!(c.end.is_none());
    }

    #[test]
    fn parse_same_chapter_range() {
        let c = Citation::parse("KJV/JHN/3/16-17").unwrap();
        assert_eq!(c.verse, Some(16));
        assert_eq!(c.end, Some((3, 17)));
    }

    #[test]
    fn parse_cross_chapter_range() {
        let c = Citation::parse("KJV/JHN/3/35-4/2").unwrap();
        assert_eq!(c.verse, Some(35));
        assert_eq!(c.end, Some((4, 2)));
    }

    #[test]
    fn round_trip_canonical_forms() {
        roundtrip("KJV/JHN/3");
        roundtrip("KJV/JHN/3/16");
        roundtrip("KJV/JHN/3/16-17");
        roundtrip("KJV/JHN/3/35-4/2");
        roundtrip("ONBV/PSA/23");
        roundtrip("ONBV/1CO/13/4-7");
    }

    #[test]
    fn parse_normalizes_case() {
        let c = Citation::parse("kjv/jhn/3/16").unwrap();
        assert_eq!(c.translation, "KJV");
        assert_eq!(c.book, "JHN");
    }

    #[test]
    fn rejects_bad_input() {
        assert!(Citation::parse("").is_err());
        assert!(Citation::parse("KJV").is_err());
        assert!(Citation::parse("KJV/JOHN/3/16").is_err());
        assert!(Citation::parse("KJV/JHN/x").is_err());
        assert!(Citation::parse("KJV/JHN/3/16-").is_err());
    }
}
