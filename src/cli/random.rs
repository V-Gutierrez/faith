//! `faith random` — uniformly random verse with deterministic seed support.

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::books;
use crate::cli::resolve_translation;
use crate::error::{FaithError, Result};
use crate::schema::{BookNames, ErrorOut, VerseOut, SCHEMA_VERSION};
use crate::store::Store;
use crate::translations;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    All,
    Ot,
    Nt,
}

pub fn run<W: Write>(
    store: &Store,
    translation: Option<&str>,
    book: Option<&str>,
    scope: Scope,
    seed: Option<u64>,
    out: &mut W,
) -> Result<i32> {
    let alias = translation.unwrap_or("KJV");
    let def = match resolve_translation(alias) {
        Ok(d) => d,
        Err(e) => return emit_err(out, &e),
    };

    let book_canonical = match book {
        Some(b) => match crate::cli::info_canonical_book(b) {
            Some(c) => Some(c),
            None => {
                let e = FaithError::RefParse {
                    input: b.to_string(),
                };
                return emit_err(out, &e);
            }
        },
        None => None,
    };

    let scope_books: Vec<&'static str> = match scope {
        Scope::All => Vec::new(),
        Scope::Ot => books::all()
            .iter()
            .filter(|b| b.testament == "OT")
            .map(|b| b.canonical_id)
            .collect(),
        Scope::Nt => books::all()
            .iter()
            .filter(|b| b.testament == "NT")
            .map(|b| b.canonical_id)
            .collect(),
    };

    let nth = seed.unwrap_or_else(default_seed);

    let book_in: Option<&[&str]> = if scope_books.is_empty() {
        None
    } else {
        Some(scope_books.as_slice())
    };

    let picked = match store.random_verse(def.alias, book_canonical.as_deref(), book_in, nth) {
        Ok(Some(v)) => v,
        Ok(None) => {
            let e = FaithError::NotFound {
                reference: format!("{}/<random>", def.alias),
            };
            return emit_err(out, &e);
        }
        Err(e) => return emit_err(out, &e),
    };

    let (book_id, chapter, verse, text) = picked;
    let book_entry = books::by_canonical_id(&book_id);
    let book_name = book_entry.map(|b| BookNames {
        en: Some(b.name_en.to_string()),
        pt: Some(b.name_pt.to_string()),
    });

    let v = VerseOut {
        schema: SCHEMA_VERSION,
        reference: format!("{}/{}/{}/{}", def.alias, book_id, chapter, verse),
        translation: def.alias.to_string(),
        book: book_id,
        book_name,
        chapter,
        verse,
        text,
        lang: translations::lang_code_to_iso2(def.language).to_string(),
        dir: def.direction.to_string(),
    };

    serde_json::to_writer(&mut *out, &v)?;
    writeln!(out)?;
    Ok(0)
}

fn emit_err<W: Write>(out: &mut W, e: &FaithError) -> Result<i32> {
    let eo = ErrorOut::from_err(e);
    serde_json::to_writer(&mut *out, &eo)?;
    writeln!(out)?;
    Ok(e.exit_code_int())
}

fn default_seed() -> u64 {
    if let Ok(s) = std::env::var("FAITH_SEED") {
        if let Ok(n) = s.parse::<u64>() {
            return n;
        }
    }
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0xCAFE_BABE_DEAD_BEEF)
}
