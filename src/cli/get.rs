use std::io::Write;

use crate::cli::{lookup, render_text, resolve_translation, tabular, OutputFormat};
use crate::error::Result;
use crate::reference;
use crate::schema::LookupOut;
use crate::store::Store;

/// Run the `get` subcommand against `store` for the given reference and
/// translation aliases, writing rendered output to `out` in `format`.
///
/// Errors per-translation are reported inline; the returned exit code is the
/// worst error encountered (0 on success).
pub fn run<W: Write>(
    store: &Store,
    reference_input: &str,
    translations: &[String],
    format: OutputFormat,
    out: &mut W,
) -> Result<i32> {
    let parsed = match reference::parse(reference_input) {
        Ok(p) => p,
        Err(e) => {
            let err = crate::schema::ErrorOut::from_err(&e);
            emit_single(out, &LookupOut::Error(err), format)?;
            return Ok(e.exit_code_int());
        }
    };

    let trs: Vec<String> = if translations.is_empty() {
        vec!["KJV".to_string()]
    } else {
        translations
            .iter()
            .flat_map(|s| s.split(','))
            .map(|s| s.trim().to_string())
            .collect()
    };

    let mut results: Vec<LookupOut> = Vec::with_capacity(trs.len());
    let mut worst_exit = 0i32;
    for alias in &trs {
        match resolve_translation(alias) {
            Ok(def) => {
                let r = lookup(store, &parsed, def);
                if let LookupOut::Error(ref e) = r {
                    worst_exit = worst_exit.max(exit_for_code(&e.error.code));
                }
                results.push(r);
            }
            Err(e) => {
                worst_exit = worst_exit.max(e.exit_code_int());
                results.push(LookupOut::Error(crate::schema::ErrorOut::from_err(&e)));
            }
        }
    }

    match format {
        OutputFormat::Tsv | OutputFormat::Csv => {
            let csv = matches!(format, OutputFormat::Csv);
            tabular::write_verse_header(out, csv)?;
            tabular::write_lookup_rows(out, &results, csv)?;
        }
        OutputFormat::Text => {
            if results.len() == 1 {
                writeln!(out, "{}", render_text(&results[0]))?;
            } else {
                for (i, r) in results.iter().enumerate() {
                    if i > 0 {
                        writeln!(out)?;
                    }
                    writeln!(out, "{}", render_text(r))?;
                }
            }
        }
        OutputFormat::Json => {
            if results.len() == 1 {
                serde_json::to_writer(&mut *out, &results[0])?;
                writeln!(out)?;
            } else {
                serde_json::to_writer(&mut *out, &results)?;
                writeln!(out)?;
            }
        }
    }
    Ok(worst_exit)
}

fn emit_single<W: Write>(out: &mut W, r: &LookupOut, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Text => {
            writeln!(out, "{}", render_text(r))?;
        }
        OutputFormat::Tsv | OutputFormat::Csv => {
            let csv = matches!(format, OutputFormat::Csv);
            tabular::write_verse_header(out, csv)?;
            tabular::write_lookup_rows(out, std::slice::from_ref(r), csv)?;
        }
        OutputFormat::Json => {
            serde_json::to_writer(&mut *out, r)?;
            writeln!(out)?;
        }
    }
    Ok(())
}

fn exit_for_code(c: &crate::error::ErrorCode) -> i32 {
    match c {
        crate::error::ErrorCode::RefParse
        | crate::error::ErrorCode::RangeTooLarge
        | crate::error::ErrorCode::FormatUnsupported => 2,
        crate::error::ErrorCode::NotFound => 3,
        crate::error::ErrorCode::TranslationMissing | crate::error::ErrorCode::DataMissing => 4,
        crate::error::ErrorCode::Io => 5,
    }
}
