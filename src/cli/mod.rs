//! CLI subcommand modules and shared lookup helpers.

use crate::books;
use crate::error::{FaithError, Result};
use crate::reference::ParsedRef;
use crate::schema::{
    BookNames, ErrorOut, LookupOut, RangeOut, VerseLite, VerseOut, SCHEMA_VERSION,
};
use crate::store::Store;
use crate::translations::{self, TranslationDef};

pub const MAX_RANGE_VERSES: usize = 500;

pub mod batch;
pub mod diff;
pub mod get;
pub mod info;
pub mod install;
pub mod list;
pub mod manifest;
pub mod random;
pub mod stats;

pub fn resolve_translation(alias: &str) -> Result<&'static TranslationDef> {
    translations::by_alias(alias).ok_or_else(|| FaithError::TranslationMissing {
        translation: alias.to_string(),
    })
}

pub fn info_canonical_book(input: &str) -> Option<String> {
    if let Some(b) = books::by_canonical_id(input) {
        return Some(b.canonical_id.to_string());
    }
    let key = crate::reference::normalize(input);
    books::alias_index().get(&key).map(|c| c.to_string())
}

pub fn lookup(store: &Store, parsed: &ParsedRef, def: &TranslationDef) -> LookupOut {
    match do_lookup(store, parsed, def) {
        Ok(out) => out,
        Err(e) => LookupOut::Error(ErrorOut::from_err(&e)),
    }
}

fn do_lookup(store: &Store, parsed: &ParsedRef, def: &TranslationDef) -> Result<LookupOut> {
    store.require_translation(def.alias)?;
    let book_entry = books::by_canonical_id(&parsed.book).ok_or_else(|| FaithError::NotFound {
        reference: format!("{}/{}", def.alias, parsed.book),
    })?;
    let book_name = Some(BookNames {
        en: Some(book_entry.name_en.to_string()),
        pt: Some(book_entry.name_pt.to_string()),
    });
    let lang = translations::lang_code_to_iso2(def.language).to_string();
    let dir = def.direction.to_string();

    match (parsed.verse, parsed.end_verse) {
        (Some(v), None) => {
            let text = store
                .get_verse(def.alias, &parsed.book, parsed.chapter, v)?
                .ok_or_else(|| FaithError::NotFound {
                    reference: format!("{}/{}/{}/{}", def.alias, parsed.book, parsed.chapter, v),
                })?;
            Ok(LookupOut::Verse(VerseOut {
                schema: SCHEMA_VERSION,
                reference: format!("{}/{}/{}/{}", def.alias, parsed.book, parsed.chapter, v),
                translation: def.alias.to_string(),
                book: parsed.book.clone(),
                book_name,
                chapter: parsed.chapter,
                verse: v,
                text,
                lang,
                dir,
            }))
        }

        (Some(v), Some(ev)) => {
            let ec = parsed.end_chapter.unwrap_or(parsed.chapter);
            let rows = store.get_range(def.alias, &parsed.book, parsed.chapter, v, ec, ev)?;
            if rows.is_empty() {
                return Err(FaithError::NotFound {
                    reference: format!(
                        "{}/{}/{}/{}-{}",
                        def.alias, parsed.book, parsed.chapter, v, ev
                    ),
                });
            }
            if rows.len() > MAX_RANGE_VERSES {
                return Err(FaithError::RangeTooLarge {
                    requested: rows.len() as u32,
                    max: MAX_RANGE_VERSES as u32,
                });
            }
            let reference = if ec == parsed.chapter {
                format!(
                    "{}/{}/{}/{}-{}",
                    def.alias, parsed.book, parsed.chapter, v, ev
                )
            } else {
                format!(
                    "{}/{}/{}/{}-{}/{}",
                    def.alias, parsed.book, parsed.chapter, v, ec, ev
                )
            };
            let chapter_field = if ec == parsed.chapter {
                Some(parsed.chapter)
            } else {
                None
            };
            let verses = rows
                .into_iter()
                .map(|(c, vn, t)| VerseLite {
                    chapter: c,
                    verse: vn,
                    text: t,
                })
                .collect();
            Ok(LookupOut::Range(RangeOut {
                schema: SCHEMA_VERSION,
                reference,
                translation: def.alias.to_string(),
                book: parsed.book.clone(),
                book_name,
                chapter: chapter_field,
                lang,
                dir,
                verses,
            }))
        }

        (None, _) => {
            let rows = store.get_chapter(def.alias, &parsed.book, parsed.chapter)?;
            if rows.is_empty() {
                return Err(FaithError::NotFound {
                    reference: format!("{}/{}/{}", def.alias, parsed.book, parsed.chapter),
                });
            }
            let verses = rows
                .into_iter()
                .map(|(vn, t)| VerseLite {
                    chapter: parsed.chapter,
                    verse: vn,
                    text: t,
                })
                .collect();
            Ok(LookupOut::Range(RangeOut {
                schema: SCHEMA_VERSION,
                reference: format!("{}/{}/{}", def.alias, parsed.book, parsed.chapter),
                translation: def.alias.to_string(),
                book: parsed.book.clone(),
                book_name,
                chapter: Some(parsed.chapter),
                lang,
                dir,
                verses,
            }))
        }
    }
}

pub fn render_text(out: &LookupOut) -> String {
    match out {
        LookupOut::Verse(v) => format!("{} {}:{}  {}", v.book, v.chapter, v.verse, v.text),
        LookupOut::Range(r) => {
            let mut s = String::new();
            s.push_str(&format!("{}\n", r.reference));
            for v in &r.verses {
                s.push_str(&format!("{}:{}  {}\n", v.chapter, v.verse, v.text));
            }
            s.trim_end().to_string()
        }
        LookupOut::Error(e) => format!(
            "ERROR {}: {}",
            error_code_str(&e.error.code),
            e.error.message
        ),
    }
}

fn error_code_str(c: &crate::error::ErrorCode) -> &'static str {
    c.as_str()
}
