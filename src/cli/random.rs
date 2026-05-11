//! `faith random` — uniformly random verse with deterministic seed support.

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::books;
use crate::cli::{render_text, resolve_translation, tabular, OutputFormat};
use crate::error::{FaithError, Result};
use crate::schema::{BookNames, ErrorOut, LookupOut, VerseOut, SCHEMA_VERSION};
use crate::store::Store;
use crate::translations;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    All,
    Ot,
    Nt,
}

/// Run the `random` subcommand. `seed` overrides the default seed source
/// (env `FAITH_SEED` or wall-clock nanos) for deterministic output.
///
/// Resolution priority: `--tr` > `--lang` > default `KJV`.
/// `--lang` accepts ISO 639-2 (`pt`, `en`) or ISO 639-3 (`por`, `eng`).
pub fn run<W: Write>(
    store: &Store,
    translation: Option<&str>,
    lang: Option<&str>,
    book: Option<&str>,
    scope: Scope,
    seed: Option<u64>,
    format: OutputFormat,
    out: &mut W,
) -> Result<i32> {
    let alias = match (translation, lang) {
        (Some(t), _) => t.to_string(),
        (None, Some(l)) => match super::resolve_by_lang(l) {
            Some(a) => a.to_string(),
            None => {
                let e = FaithError::TranslationMissing {
                    translation: format!("no translation found for lang '{l}'"),
                };
                return emit_err(out, &e, format);
            }
        },
        _ => "KJV".to_string(),
    };
    let def = match resolve_translation(&alias) {
        Ok(d) => d,
        Err(e) => return emit_err(out, &e, format),
    };

    let book_canonical = match book {
        Some(b) => match crate::cli::info_canonical_book(b) {
            Some(c) => Some(c),
            None => {
                let e = FaithError::RefParse {
                    input: b.to_string(),
                };
                return emit_err(out, &e, format);
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
            return emit_err(out, &e, format);
        }
        Err(e) => return emit_err(out, &e, format),
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

    match format {
        OutputFormat::Tsv | OutputFormat::Csv => {
            let csv = matches!(format, OutputFormat::Csv);
            tabular::write_verse_header(out, csv)?;
            tabular::write_lookup_rows(out, std::slice::from_ref(&LookupOut::Verse(v)), csv)?;
        }
        OutputFormat::Text => {
            writeln!(out, "{}", render_text(&LookupOut::Verse(v)))?;
        }
        OutputFormat::Json => {
            serde_json::to_writer(&mut *out, &v)?;
            writeln!(out)?;
        }
    }
    Ok(0)
}

fn emit_err<W: Write>(out: &mut W, e: &FaithError, format: OutputFormat) -> Result<i32> {
    let eo = ErrorOut::from_err(e);
    match format {
        OutputFormat::Text => {
            writeln!(out, "{}", render_text(&LookupOut::Error(eo)))?;
        }
        _ => {
            serde_json::to_writer(&mut *out, &eo)?;
            writeln!(out)?;
        }
    }
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
