use std::io::Write;

use crate::error::Result;
use crate::schema::{AvailableTranslation, Manifest, TranslationInfo, SCHEMA_VERSION};
use crate::store::{self, Store};
use crate::translations;

pub fn run<W: Write>(store: &Store, out: &mut W) -> Result<i32> {
    let installed = store.list_translations()?;
    let installed_ids: Vec<String> = installed.iter().map(|t| t.id.clone()).collect();

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

    let available_translations: Vec<AvailableTranslation> = translations::CATALOG
        .iter()
        .filter(|t| !installed_ids.iter().any(|id| id == t.alias))
        .map(|t| AvailableTranslation {
            alias: t.alias.to_string(),
            name: t.name.to_string(),
            language: t.language.to_string(),
            source_url: t.source_url.to_string(),
        })
        .collect();

    let manifest = Manifest {
        schema: SCHEMA_VERSION,
        version: env!("CARGO_PKG_VERSION").to_string(),
        data_dir: store::data_dir()?.to_string_lossy().into_owned(),
        translations,
        available_translations,
        tools: crate::schema::tool_inventory_v1(),
    };

    serde_json::to_writer(&mut *out, &manifest)?;
    writeln!(out)?;
    Ok(0)
}
