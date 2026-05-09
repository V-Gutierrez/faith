//! `faith diff` — compare two or more translations for the same reference.
//!
//! Output is a single `DiffOut` JSON object listing each requested translation
//! as a `DiffEntry`. Per-entry errors (missing translation, not found) are
//! reported inline so callers always receive a well-formed envelope.

use std::io::Write;

use crate::cli::{lookup, resolve_translation};
use crate::error::{ErrorCode, FaithError, Result};
use crate::reference;
use crate::schema::{DiffEntry, DiffOut, ErrorBody, ErrorOut, LookupOut, SCHEMA_VERSION};
use crate::store::Store;

/// Run the `diff` subcommand.
///
/// Requires at least two translation aliases; otherwise emits an error
/// envelope (exit code 2). Each translation is looked up independently and
/// reported as its own `DiffEntry` so partial failures do not abort the diff.
pub fn run<W: Write>(
    store: &Store,
    reference_input: &str,
    translations: &[String],
    out: &mut W,
) -> Result<i32> {
    let trs: Vec<String> = translations
        .iter()
        .flat_map(|s| s.split(','))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if trs.len() < 2 {
        let e = FaithError::RefParse {
            input: format!("diff requires at least two translations, got {}", trs.len()),
        };
        let eo = ErrorOut::from_err(&e);
        serde_json::to_writer(&mut *out, &eo)?;
        writeln!(out)?;
        return Ok(e.exit_code_int());
    }

    let parsed = match reference::parse(reference_input) {
        Ok(p) => p,
        Err(e) => {
            let eo = ErrorOut::from_err(&e);
            serde_json::to_writer(&mut *out, &eo)?;
            writeln!(out)?;
            return Ok(e.exit_code_int());
        }
    };

    let mut entries: Vec<DiffEntry> = Vec::with_capacity(trs.len());
    for alias in &trs {
        let entry = match resolve_translation(alias) {
            Ok(def) => match lookup(store, &parsed, def) {
                LookupOut::Verse(v) => DiffEntry {
                    id: def.alias.to_string(),
                    text: Some(v.text),
                    verses: None,
                    error: None,
                },
                LookupOut::Range(r) => DiffEntry {
                    id: def.alias.to_string(),
                    text: None,
                    verses: Some(r.verses),
                    error: None,
                },
                LookupOut::Error(e) => DiffEntry {
                    id: def.alias.to_string(),
                    text: None,
                    verses: None,
                    error: Some(e.error),
                },
            },
            Err(e) => DiffEntry {
                id: alias.clone(),
                text: None,
                verses: None,
                error: Some(ErrorBody {
                    code: ErrorCode::TranslationMissing,
                    message: e.to_string(),
                    input: e.input().map(str::to_owned),
                }),
            },
        };
        entries.push(entry);
    }

    let diff = DiffOut {
        schema: SCHEMA_VERSION,
        kind: "diff",
        reference: reference_input.to_string(),
        translations: entries,
    };

    serde_json::to_writer(&mut *out, &diff)?;
    writeln!(out)?;
    Ok(0)
}
