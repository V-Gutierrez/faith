//! `faith diff` — compare two or more translations for the same reference.
//!
//! Output is a single `DiffOut` JSON object listing each requested translation
//! as a `DiffEntry`. Per-entry errors (missing translation, not found) are
//! reported inline so callers always receive a well-formed envelope.

use std::io::Write;

use crate::cli::{lookup, resolve_translation, tabular, OutputFormat};
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
    format: OutputFormat,
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
    let mut lookups: Vec<LookupOut> = Vec::with_capacity(trs.len());
    for alias in &trs {
        let lookup_out = match resolve_translation(alias) {
            Ok(def) => lookup(store, &parsed, def),
            Err(e) => LookupOut::Error(ErrorOut {
                schema: SCHEMA_VERSION,
                error: ErrorBody {
                    code: ErrorCode::TranslationMissing,
                    message: e.to_string(),
                    input: e.input().map(str::to_owned),
                },
            }),
        };
        let entry = match &lookup_out {
            LookupOut::Verse(v) => DiffEntry {
                id: v.translation.clone(),
                text: Some(v.text.clone()),
                verses: None,
                error: None,
            },
            LookupOut::Range(r) => DiffEntry {
                id: r.translation.clone(),
                text: None,
                verses: Some(r.verses.clone()),
                error: None,
            },
            LookupOut::Error(e) => DiffEntry {
                id: alias.clone(),
                text: None,
                verses: None,
                error: Some(e.error.clone()),
            },
        };
        entries.push(entry);
        lookups.push(lookup_out);
    }

    match format {
        OutputFormat::Tsv | OutputFormat::Csv => {
            let csv = matches!(format, OutputFormat::Csv);
            tabular::write_verse_header(out, csv)?;
            tabular::write_lookup_rows(out, &lookups, csv)?;
        }
        OutputFormat::Text => {
            for (i, e) in entries.iter().enumerate() {
                if i > 0 {
                    writeln!(out)?;
                }
                if let Some(t) = &e.text {
                    writeln!(out, "{}  {}", e.id, t)?;
                } else if let Some(vs) = &e.verses {
                    writeln!(out, "{}", e.id)?;
                    for v in vs {
                        writeln!(out, "  {}:{}  {}", v.chapter, v.verse, v.text)?;
                    }
                } else if let Some(err) = &e.error {
                    writeln!(
                        out,
                        "{}  ERROR {}: {}",
                        e.id,
                        err.code.as_str(),
                        err.message
                    )?;
                }
            }
        }
        OutputFormat::Json => {
            let diff = DiffOut {
                schema: SCHEMA_VERSION,
                kind: "diff",
                reference: reference_input.to_string(),
                translations: entries,
            };
            serde_json::to_writer(&mut *out, &diff)?;
            writeln!(out)?;
        }
    }
    Ok(0)
}
