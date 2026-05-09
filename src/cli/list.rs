use std::io::Write;

use serde::Serialize;

use crate::cli::{tabular, OutputFormat};
use crate::error::{FaithError, Result};
use crate::store::Store;
use crate::translations;

#[derive(Debug, Serialize)]
struct TranslationListItem {
    id: String,
    name: String,
    english_name: String,
    language: String,
    direction: String,
    license: String,
    source_url: String,
    installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    installed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    books: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verses: Option<u32>,
}

const TRANSLATION_HEADERS: &[&str] = &["id", "name", "language", "direction", "verses"];
const BOOK_HEADERS: &[&str] = &["usfm"];

/// List installed and available translations.
pub fn run_translations<W: Write>(
    store: &Store,
    lang_filter: Option<&str>,
    only_installed: bool,
    format: OutputFormat,
    out: &mut W,
) -> Result<i32> {
    let installed = store.list_translations()?;
    let mut items: Vec<TranslationListItem> = Vec::new();
    for def in translations::CATALOG {
        if let Some(lf) = lang_filter {
            if def.language != lf {
                continue;
            }
        }
        let inst = installed.iter().find(|t| t.id == def.alias);
        let is_installed = inst.is_some();
        if only_installed && !is_installed {
            continue;
        }
        items.push(TranslationListItem {
            id: def.alias.to_string(),
            name: def.name.to_string(),
            english_name: def.english_name.to_string(),
            language: def.language.to_string(),
            direction: def.direction.to_string(),
            license: def.license.to_string(),
            source_url: def.source_url.to_string(),
            installed: is_installed,
            installed_at: inst.map(|i| i.installed_at.clone()),
            books: inst.map(|i| i.books),
            verses: inst.map(|i| i.verses),
        });
    }
    match format {
        OutputFormat::Tsv | OutputFormat::Csv => {
            let csv = matches!(format, OutputFormat::Csv);
            tabular::write_row(out, TRANSLATION_HEADERS.iter().copied(), csv)?;
            for it in &items {
                let v = it.verses.map(|n| n.to_string()).unwrap_or_default();
                tabular::write_row(
                    out,
                    [
                        it.id.as_str(),
                        it.name.as_str(),
                        it.language.as_str(),
                        it.direction.as_str(),
                        v.as_str(),
                    ],
                    csv,
                )?;
            }
        }
        _ => {
            serde_json::to_writer(&mut *out, &items)?;
            writeln!(out)?;
        }
    }
    Ok(0)
}

/// List books available in `translation`.
pub fn run_books<W: Write>(
    store: &Store,
    translation: &str,
    format: OutputFormat,
    out: &mut W,
) -> Result<i32> {
    let def =
        translations::by_alias(translation).ok_or_else(|| FaithError::TranslationMissing {
            translation: translation.to_string(),
        })?;
    let books = store.list_books(def.alias)?;
    match format {
        OutputFormat::Tsv | OutputFormat::Csv => {
            let csv = matches!(format, OutputFormat::Csv);
            tabular::write_row(out, BOOK_HEADERS.iter().copied(), csv)?;
            for b in &books {
                tabular::write_row(out, [b.as_str()], csv)?;
            }
        }
        _ => {
            serde_json::to_writer(&mut *out, &books)?;
            writeln!(out)?;
        }
    }
    Ok(0)
}
