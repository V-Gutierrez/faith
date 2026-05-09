use std::io::Write;

use crate::error::Result;
use crate::schema::{Manifest, TranslationInfo, SCHEMA_VERSION};
use crate::store::{self, Store};

pub fn run<W: Write>(store: &Store, out: &mut W) -> Result<i32> {
    let installed = store.list_translations()?;
    let translations: Vec<TranslationInfo> = installed
        .into_iter()
        .map(|t| TranslationInfo {
            id: t.id,
            name: t.name,
            english_name: t.english_name,
            language: t.language,
            direction: t.direction,
            books: t.books,
            verses: t.verses,
            license: t.license,
            source_url: t.source_url,
            installed_at: t.installed_at,
        })
        .collect();

    let manifest = Manifest {
        schema: SCHEMA_VERSION,
        version: env!("CARGO_PKG_VERSION").to_string(),
        data_dir: store::data_dir()?.to_string_lossy().into_owned(),
        translations,
        tools: crate::schema::tool_inventory_v1(),
    };

    serde_json::to_writer(&mut *out, &manifest)?;
    writeln!(out)?;
    Ok(0)
}
