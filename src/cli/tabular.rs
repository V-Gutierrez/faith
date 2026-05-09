//! Tabular row formatting for `--format tsv` / `--format csv`.
//!
//! Header conventions (kept stable; additions go at the end):
//! - Verse-shaped output: `translation,book,chapter,verse,text`
//! - Translation list:    `id,name,language,direction,verses`
//! - Book list:           `usfm`
//!
//! TSV writes one row per line with `\t` separators; literal tabs/newlines in
//! `text` are converted to single spaces so each row stays atomic.
//! CSV follows RFC 4180: comma separator, fields containing `,`, `"`, `\r`, or
//! `\n` are quoted with embedded `"` doubled.

use std::io::Write;

use crate::error::Result;
use crate::schema::LookupOut;

pub const VERSE_HEADERS: &[&str] = &["translation", "book", "chapter", "verse", "text"];

pub fn write_verse_header<W: Write>(out: &mut W, csv: bool) -> Result<()> {
    write_row(out, VERSE_HEADERS.iter().copied(), csv)
}

pub fn write_lookup_rows<W: Write>(out: &mut W, lookups: &[LookupOut], csv: bool) -> Result<()> {
    for r in lookups {
        match r {
            LookupOut::Verse(v) => {
                let chap = v.chapter.to_string();
                let vno = v.verse.to_string();
                write_row(
                    out,
                    [
                        v.translation.as_str(),
                        v.book.as_str(),
                        &chap,
                        &vno,
                        v.text.as_str(),
                    ],
                    csv,
                )?;
            }
            LookupOut::Range(rng) => {
                for vl in &rng.verses {
                    let chap = vl.chapter.to_string();
                    let vno = vl.verse.to_string();
                    write_row(
                        out,
                        [
                            rng.translation.as_str(),
                            rng.book.as_str(),
                            &chap,
                            &vno,
                            vl.text.as_str(),
                        ],
                        csv,
                    )?;
                }
            }
            LookupOut::Error(_) => {
                // Errors are skipped from tabular output by design (no
                // unambiguous row shape); JSON callers still get them.
            }
        }
    }
    Ok(())
}

pub fn write_row<'a, W, I>(out: &mut W, fields: I, csv: bool) -> Result<()>
where
    W: Write,
    I: IntoIterator<Item = &'a str>,
{
    let sep = if csv { ',' } else { '\t' };
    let mut first = true;
    for f in fields {
        if !first {
            write!(out, "{sep}")?;
        }
        first = false;
        if csv {
            write!(out, "{}", csv_escape(f))?;
        } else {
            write!(out, "{}", tsv_sanitize(f))?;
        }
    }
    writeln!(out)?;
    Ok(())
}

fn tsv_sanitize(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

fn csv_escape(s: &str) -> String {
    let needs_quote = s.contains([',', '"', '\n', '\r']);
    if !needs_quote {
        return s.to_string();
    }
    let escaped = s.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_escape_quotes_when_needed() {
        assert_eq!(csv_escape("plain"), "plain");
        assert_eq!(csv_escape("a,b"), "\"a,b\"");
        assert_eq!(csv_escape("a\"b"), "\"a\"\"b\"");
        assert_eq!(csv_escape("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn tsv_sanitize_replaces_whitespace() {
        assert_eq!(tsv_sanitize("a\tb\nc\rd"), "a b c d");
    }
}
