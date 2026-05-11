//! `faith.v1` JSON output types.
//!
//! These types are the public output contract. Field renames or removals are
//! breaking; additive optional fields are not.

use serde::Serialize;

use crate::error::ErrorCode;

pub const SCHEMA_VERSION: &str = "faith.v1";

#[derive(Debug, Clone, Serialize)]
pub struct VerseOut {
    pub schema: &'static str,
    #[serde(rename = "ref")]
    pub reference: String,
    pub translation: String,
    pub book: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_name: Option<BookNames>,
    pub chapter: u16,
    pub verse: u16,
    pub text: String,
    pub lang: String,
    pub dir: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct BookNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RangeOut {
    pub schema: &'static str,
    #[serde(rename = "ref")]
    pub reference: String,
    pub translation: String,
    pub book: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_name: Option<BookNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<u16>,
    pub lang: String,
    pub dir: String,
    pub verses: Vec<VerseLite>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerseLite {
    pub chapter: u16,
    pub verse: u16,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LookupOut {
    Verse(VerseOut),
    Range(RangeOut),
    Error(ErrorOut),
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorOut {
    pub schema: &'static str,
    pub error: ErrorBody,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorBody {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

impl ErrorOut {
    pub fn from_err(err: &crate::error::FaithError) -> Self {
        Self {
            schema: SCHEMA_VERSION,
            error: ErrorBody {
                code: err.code(),
                message: err.to_string(),
                input: err.input().map(str::to_owned),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Manifest {
    pub schema: &'static str,
    pub version: String,
    pub data_dir: String,
    pub translations: Vec<TranslationInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_translations: Vec<AvailableTranslation>,
    pub tools: Vec<ToolInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TranslationInfo {
    pub id: String,
    pub name: String,
    pub english_name: String,
    pub language: String,
    pub direction: String,
    pub books: u16,
    pub verses: u32,
    pub license: String,
    pub source_url: String,
    pub installed_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub args: Vec<String>,
}

/// Catalog entry not yet installed — shown in `faith manifest` so agents
/// know what they *can* install.
#[derive(Debug, Clone, Serialize)]
pub struct AvailableTranslation {
    pub alias: String,
    pub name: String,
    pub language: String,
    pub source_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookInfoOut {
    pub schema: &'static str,
    pub kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
    pub book: BookInfoBody,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookInfoBody {
    pub usfm: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_name: Option<BookNames>,
    pub aliases: Vec<String>,
    pub chapters: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verses_total: Option<u32>,
    pub testament: &'static str,
    pub order: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffOut {
    pub schema: &'static str,
    pub kind: &'static str,
    #[serde(rename = "ref")]
    pub reference: String,
    pub translations: Vec<DiffEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffEntry {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verses: Option<Vec<VerseLite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GlobalStatsOut {
    pub schema: &'static str,
    pub kind: &'static str,
    pub translations_installed: u16,
    pub total_verses: u64,
    pub db_size_bytes: u64,
    pub cache_size_bytes: u64,
    pub manifest_last_updated: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TranslationStatsOut {
    pub schema: &'static str,
    pub kind: &'static str,
    pub translation: String,
    pub language: String,
    pub books: u16,
    pub chapters: u16,
    pub verses: u32,
    pub ot_verses: u32,
    pub nt_verses: u32,
    pub installed_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CacheStatsOut {
    pub schema: &'static str,
    pub kind: &'static str,
    pub db_bytes: u64,
    pub cache_bytes: u64,
    pub manifest_bytes: u64,
    pub total_bytes: u64,
    pub path: String,
}

/// Generic structured message (used by `cache clear`, `cache path`, etc.).
#[derive(Debug, Clone, Serialize)]
pub struct MessageOut {
    pub schema: &'static str,
    pub kind: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchOut {
    pub schema: &'static str,
    pub kind: &'static str,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
    pub matches: Vec<SearchMatch>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchMatch {
    #[serde(rename = "ref")]
    pub reference: String,
    pub translation: String,
    pub book: String,
    pub chapter: u16,
    pub verse: u16,
    pub snippet: String,
    pub rank: f64,
}

pub fn tool_inventory_v1() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "get".into(),
            args: vec!["ref".into(), "tr?".into(), "lang?".into()],
        },
        ToolInfo {
            name: "batch".into(),
            args: vec!["tr?".into(), "stdin: refs[]".into()],
        },
        ToolInfo {
            name: "list".into(),
            args: vec!["kind".into()],
        },
        ToolInfo {
            name: "install".into(),
            args: vec!["tr+".into()],
        },
        ToolInfo {
            name: "manifest".into(),
            args: vec![],
        },
        ToolInfo {
            name: "info".into(),
            args: vec!["book".into(), "tr?".into()],
        },
        ToolInfo {
            name: "random".into(),
            args: vec![
                "tr?".into(),
                "lang?".into(),
                "book?".into(),
                "scope?".into(),
            ],
        },
        ToolInfo {
            name: "diff".into(),
            args: vec!["ref".into(), "tr+".into(), "lang?".into()],
        },
        ToolInfo {
            name: "stats".into(),
            args: vec!["tr?".into()],
        },
        ToolInfo {
            name: "search".into(),
            args: vec![
                "query".into(),
                "tr?".into(),
                "lang?".into(),
                "limit?".into(),
            ],
        },
        ToolInfo {
            name: "completions".into(),
            args: vec!["shell".into()],
        },
        ToolInfo {
            name: "cache".into(),
            args: vec!["sub".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verse_serializes_with_ref_field_and_schema() {
        let v = VerseOut {
            schema: SCHEMA_VERSION,
            reference: "KJV/JHN/3/16".into(),
            translation: "KJV".into(),
            book: "JHN".into(),
            book_name: Some(BookNames {
                en: Some("John".into()),
                pt: Some("João".into()),
            }),
            chapter: 3,
            verse: 16,
            text: "For God so loved the world".into(),
            lang: "en".into(),
            dir: "ltr".into(),
        };
        let s = serde_json::to_string(&v).unwrap();
        assert!(s.contains("\"schema\":\"faith.v1\""));
        assert!(s.contains("\"ref\":\"KJV/JHN/3/16\""));
        assert!(s.contains("\"book_name\":{\"en\":\"John\",\"pt\":\"João\"}"));
    }

    #[test]
    fn missing_book_name_is_omitted() {
        let v = VerseOut {
            schema: SCHEMA_VERSION,
            reference: "KJV/JHN/3/16".into(),
            translation: "KJV".into(),
            book: "JHN".into(),
            book_name: None,
            chapter: 3,
            verse: 16,
            text: "x".into(),
            lang: "en".into(),
            dir: "ltr".into(),
        };
        let s = serde_json::to_string(&v).unwrap();
        assert!(!s.contains("book_name"));
    }
}
