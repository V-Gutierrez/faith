use std::io::{Read, Write};

use crate::cli::{lookup, render_text, resolve_translation};
use crate::error::Result;
use crate::reference;
use crate::schema::{ErrorOut, LookupOut};
use crate::store::Store;

pub fn run<R: Read, W: Write>(
    store: &Store,
    translation: &str,
    format_text: bool,
    stdin: &mut R,
    out: &mut W,
) -> Result<i32> {
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;
    let refs: Vec<String> =
        serde_json::from_str(&buf).map_err(|e| crate::error::FaithError::RefParse {
            input: format!("stdin: {e}"),
        })?;

    let def = resolve_translation(translation)?;
    let mut results: Vec<LookupOut> = Vec::with_capacity(refs.len());
    let mut worst = 0i32;
    for r in &refs {
        match reference::parse(r) {
            Ok(parsed) => {
                let res = lookup(store, &parsed, def);
                if let LookupOut::Error(ref e) = res {
                    worst = worst.max(exit_for_code(&e.error.code));
                }
                results.push(res);
            }
            Err(e) => {
                worst = worst.max(e.exit_code_int());
                results.push(LookupOut::Error(ErrorOut::from_err(&e)));
            }
        }
    }

    if format_text {
        for (i, r) in results.iter().enumerate() {
            if i > 0 {
                writeln!(out)?;
            }
            writeln!(out, "{}", render_text(r))?;
        }
    } else {
        serde_json::to_writer(&mut *out, &results)?;
        writeln!(out)?;
    }
    Ok(worst)
}

fn exit_for_code(c: &crate::error::ErrorCode) -> i32 {
    match c {
        crate::error::ErrorCode::RefParse => 2,
        crate::error::ErrorCode::NotFound => 3,
        crate::error::ErrorCode::TranslationMissing | crate::error::ErrorCode::DataMissing => 4,
        crate::error::ErrorCode::Io => 5,
    }
}
