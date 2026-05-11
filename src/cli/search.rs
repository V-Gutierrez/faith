//! `faith search` — full-text search across installed translations.
//!
//! Uses the FTS5 index populated at install time. Results are ranked by BM25
//! relevance. Snippets use `»…«` markers for highlighted matches.

use std::io::Write;

use crate::cli::OutputFormat;
use crate::error::Result;
use crate::schema::{SearchMatch, SearchOut, SCHEMA_VERSION};
use crate::store::Store;

const DEFAULT_LIMIT: u32 = 20;

/// Run the `search` subcommand.
///
/// `translation` filters to a single installed translation; when `None`,
/// searches across all installed data.
pub fn run<W: Write>(
    store: &Store,
    query: &str,
    translation: Option<&str>,
    limit: Option<u32>,
    format: OutputFormat,
    out: &mut W,
) -> Result<i32> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(200);
    let hits = store.search_verses(query, translation, limit)?;

    let matches: Vec<SearchMatch> = hits
        .into_iter()
        .map(|h| SearchMatch {
            reference: format!("{}/{}/{}/{}", h.translation, h.book, h.chapter, h.verse),
            translation: h.translation,
            book: h.book,
            chapter: h.chapter,
            verse: h.verse,
            snippet: h.snippet,
            rank: h.rank,
        })
        .collect();

    let total = matches.len();
    let search_out = SearchOut {
        schema: SCHEMA_VERSION,
        kind: "search",
        query: query.to_string(),
        translation: translation.map(str::to_owned),
        matches,
        total,
    };

    match format {
        OutputFormat::Json => {
            serde_json::to_writer(&mut *out, &search_out)?;
            writeln!(out)?;
        }
        OutputFormat::Text => {
            writeln!(out, "Search: \"{}\" ({} results)", query, total)?;
            for m in &search_out.matches {
                writeln!(out, "  {} — {}", m.reference, m.snippet)?;
            }
        }
        OutputFormat::Tsv | OutputFormat::Csv => {
            let sep = if matches!(format, OutputFormat::Csv) {
                ","
            } else {
                "\t"
            };
            writeln!(
                out,
                "translation{}book{}chapter{}verse{}snippet",
                sep, sep, sep, sep
            )?;
            for m in &search_out.matches {
                let snippet = if matches!(format, OutputFormat::Csv) {
                    format!("\"{}\"", m.snippet.replace('"', "\"\""))
                } else {
                    m.snippet.replace('\t', " ").replace('\n', " ")
                };
                writeln!(
                    out,
                    "{}{}{}{}{}{}{}{}{}",
                    m.translation, sep, m.book, sep, m.chapter, sep, m.verse, sep, snippet
                )?;
            }
        }
    }

    Ok(0)
}
