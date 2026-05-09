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

pub fn tool_inventory_v1() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "get".into(),
            args: vec!["ref".into(), "tr?".into()],
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
