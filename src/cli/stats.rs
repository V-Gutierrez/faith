//! `faith stats` — observability over the installed dataset.
//!
//! With no `--tr`, emits a `GlobalStatsOut` (translation count, total verses,
//! db/cache sizes, manifest mtime). With `--tr`, emits a `TranslationStatsOut`
//! (book/chapter/verse counts plus OT/NT split derived from `crate::books`).

use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::books;
use crate::cli::resolve_translation;
use crate::error::{FaithError, Result};
use crate::schema::{ErrorOut, GlobalStatsOut, TranslationStatsOut, SCHEMA_VERSION};
use crate::store::Store;

/// Run the `stats` subcommand.
///
/// `data_dir` is injected so tests can point at a tempdir; `main` passes the
/// resolved `store::data_dir()`.
pub fn run<W: Write>(
    store: &Store,
    translation: Option<&str>,
    data_dir: &Path,
    out: &mut W,
) -> Result<i32> {
    match translation {
        Some(alias) => emit_translation(store, alias, out),
        None => emit_global(store, data_dir, out),
    }
}

fn emit_global<W: Write>(store: &Store, data_dir: &Path, out: &mut W) -> Result<i32> {
    let trs = store.list_translations()?;
    let total_verses: u64 = trs.iter().map(|t| t.verses as u64).sum();
    let db_size_bytes = file_size(&data_dir.join("bible.db"));
    let cache_size_bytes = dir_size(&data_dir.join("cache"));
    let manifest_last_updated = file_mtime_iso(&data_dir.join("manifest.json"));

    let g = GlobalStatsOut {
        schema: SCHEMA_VERSION,
        kind: "stats.global",
        translations_installed: trs.len() as u16,
        total_verses,
        db_size_bytes,
        cache_size_bytes,
        manifest_last_updated,
    };
    serde_json::to_writer(&mut *out, &g)?;
    writeln!(out)?;
    Ok(0)
}

fn emit_translation<W: Write>(store: &Store, alias: &str, out: &mut W) -> Result<i32> {
    let def = match resolve_translation(alias) {
        Ok(d) => d,
        Err(e) => return emit_err(out, &e),
    };
    if let Err(e) = store.require_translation(def.alias) {
        return emit_err(out, &e);
    }

    let counts = store.translation_book_counts(def.alias)?;
    let chapters = store.translation_chapter_count(def.alias)?;
    let mut ot: u32 = 0;
    let mut nt: u32 = 0;
    let mut total: u32 = 0;
    for (book_id, n) in &counts {
        total += n;
        match books::by_canonical_id(book_id).map(|b| b.testament) {
            Some("OT") => ot += n,
            Some("NT") => nt += n,
            _ => {}
        }
    }

    let installed_at = store.translation_installed_at(def.alias)?;

    let s = TranslationStatsOut {
        schema: SCHEMA_VERSION,
        kind: "stats.translation",
        translation: def.alias.to_string(),
        language: def.language.to_string(),
        books: counts.len() as u16,
        chapters: chapters.min(u16::MAX as u32) as u16,
        verses: total,
        ot_verses: ot,
        nt_verses: nt,
        installed_at,
    };
    serde_json::to_writer(&mut *out, &s)?;
    writeln!(out)?;
    Ok(0)
}

fn emit_err<W: Write>(out: &mut W, e: &FaithError) -> Result<i32> {
    let eo = ErrorOut::from_err(e);
    serde_json::to_writer(&mut *out, &eo)?;
    writeln!(out)?;
    Ok(e.exit_code_int())
}

fn file_size(p: &Path) -> u64 {
    fs::metadata(p).map(|m| m.len()).unwrap_or(0)
}

fn dir_size(p: &Path) -> u64 {
    let Ok(entries) = fs::read_dir(p) else {
        return 0;
    };
    let mut total: u64 = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else { continue };
        if meta.is_dir() {
            total += dir_size(&path);
        } else {
            total += meta.len();
        }
    }
    total
}

fn file_mtime_iso(p: &Path) -> String {
    let Ok(meta) = fs::metadata(p) else {
        return String::new();
    };
    let Ok(modified) = meta.modified() else {
        return String::new();
    };
    let Ok(d) = modified.duration_since(UNIX_EPOCH) else {
        return String::new();
    };
    format_unix_seconds_utc(d.as_secs())
}

fn format_unix_seconds_utc(secs: u64) -> String {
    // Civil from-days algorithm (Howard Hinnant, public domain).
    let days = (secs / 86_400) as i64;
    let secs_of_day = secs % 86_400;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    let hh = secs_of_day / 3600;
    let mm = (secs_of_day % 3600) / 60;
    let ss = secs_of_day % 60;
    format!("{year:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}
