use std::io::Write;

use serde::Serialize;

use crate::error::{FaithError, Result};
use crate::installer;
use crate::store::Store;
use crate::translations;

#[derive(Debug, Serialize)]
struct InstallReport {
    schema: &'static str,
    installed: Vec<InstalledOne>,
}

#[derive(Debug, Serialize)]
struct InstalledOne {
    id: String,
    verses: u32,
    source_url: String,
    license: String,
}

pub fn run<W: Write>(store: &mut Store, aliases: &[String], out: &mut W) -> Result<i32> {
    let mut report = InstallReport {
        schema: crate::schema::SCHEMA_VERSION,
        installed: Vec::with_capacity(aliases.len()),
    };

    for alias in aliases {
        let def = translations::by_alias(alias).ok_or_else(|| FaithError::TranslationMissing {
            translation: alias.clone(),
        })?;
        let n = installer::install(store, def)?;
        report.installed.push(InstalledOne {
            id: def.alias.to_string(),
            verses: n,
            source_url: def.source_url.to_string(),
            license: def.license.to_string(),
        });
    }

    serde_json::to_writer(&mut *out, &report)?;
    writeln!(out)?;
    Ok(0)
}
