//! `faith info <book> [--tr <id>]` — book metadata lookup.

use std::io::Write;

use crate::books;
use crate::cli::resolve_translation;
use crate::error::{FaithError, Result};
use crate::schema::{BookInfoBody, BookInfoOut, ErrorOut, SCHEMA_VERSION};
use crate::store::Store;

pub fn run<W: Write>(
    store: &Store,
    book_input: &str,
    translation: Option<&str>,
    out: &mut W,
) -> Result<i32> {
    let entry = match resolve_book(book_input) {
        Some(b) => b,
        None => {
            let err = FaithError::RefParse {
                input: book_input.to_string(),
            };
            let eo = ErrorOut::from_err(&err);
            serde_json::to_writer(&mut *out, &eo)?;
            writeln!(out)?;
            return Ok(err.exit_code_int());
        }
    };

    let (tr_id, verses_total) = match translation {
        Some(alias) => {
            let def = match resolve_translation(alias) {
                Ok(d) => d,
                Err(e) => {
                    let eo = ErrorOut::from_err(&e);
                    serde_json::to_writer(&mut *out, &eo)?;
                    writeln!(out)?;
                    return Ok(e.exit_code_int());
                }
            };
            match store.book_verse_count(def.alias, entry.canonical_id) {
                Ok(n) => (Some(def.alias.to_string()), Some(n)),
                Err(e) => {
                    let eo = ErrorOut::from_err(&e);
                    serde_json::to_writer(&mut *out, &eo)?;
                    writeln!(out)?;
                    return Ok(e.exit_code_int());
                }
            }
        }
        None => (None, None),
    };

    let body = BookInfoBody {
        usfm: entry.canonical_id.to_string(),
        name: entry.name_en.to_string(),
        aliases: books::aliases_lower(entry),
        chapters: entry.chapters,
        verses_total,
        testament: entry.testament,
        order: entry.order,
    };

    let info = BookInfoOut {
        schema: SCHEMA_VERSION,
        kind: "book_info",
        translation: tr_id,
        book: body,
    };

    serde_json::to_writer(&mut *out, &info)?;
    writeln!(out)?;
    Ok(0)
}

fn resolve_book(input: &str) -> Option<&'static books::BookEntry> {
    if let Some(b) = books::by_canonical_id(input) {
        return Some(b);
    }
    let key = crate::reference::normalize(input);
    let canonical = books::alias_index().get(&key)?;
    books::by_canonical_id(canonical)
}
