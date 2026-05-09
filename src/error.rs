//! Stable error model for `faith`.
//!
//! Error codes are part of the public `faith.v1` schema. Adding a code is
//! additive; renaming or removing one is a breaking change.

use std::process::ExitCode;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Stable, machine-readable error codes surfaced in JSON output.
///
/// Mapping of code → exit code is defined in [`FaithError::exit_code`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "E_REF_PARSE")]
    RefParse,
    #[serde(rename = "E_NOT_FOUND")]
    NotFound,
    #[serde(rename = "E_TRANSLATION_MISSING")]
    TranslationMissing,
    #[serde(rename = "E_DATA_MISSING")]
    DataMissing,
    #[serde(rename = "E_IO")]
    Io,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RefParse => "E_REF_PARSE",
            Self::NotFound => "E_NOT_FOUND",
            Self::TranslationMissing => "E_TRANSLATION_MISSING",
            Self::DataMissing => "E_DATA_MISSING",
            Self::Io => "E_IO",
        }
    }
}

/// Top-level error type for `faith` core and CLI.
///
/// Each variant maps 1:1 to a public [`ErrorCode`] and a process exit code
/// per `docs/SPEC.md`.
#[derive(Debug, Error)]
pub enum FaithError {
    #[error("could not parse reference: {input:?}")]
    RefParse { input: String },

    #[error("reference not found: {reference}")]
    NotFound { reference: String },

    #[error("translation not installed: {translation}")]
    TranslationMissing { translation: String },

    #[error("data missing: {0}")]
    DataMissing(String),

    #[error("I/O failure: {0}")]
    Io(String),
}

impl FaithError {
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::RefParse { .. } => ErrorCode::RefParse,
            Self::NotFound { .. } => ErrorCode::NotFound,
            Self::TranslationMissing { .. } => ErrorCode::TranslationMissing,
            Self::DataMissing(_) => ErrorCode::DataMissing,
            Self::Io(_) => ErrorCode::Io,
        }
    }

    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(self.exit_code_int() as u8)
    }

    pub fn exit_code_int(&self) -> i32 {
        match self {
            Self::RefParse { .. } => 2,
            Self::NotFound { .. } => 3,
            Self::TranslationMissing { .. } | Self::DataMissing(_) => 4,
            Self::Io(_) => 5,
        }
    }

    /// Optional `input` field for the JSON `error` object.
    pub fn input(&self) -> Option<&str> {
        match self {
            Self::RefParse { input } => Some(input.as_str()),
            Self::NotFound { reference } => Some(reference.as_str()),
            Self::TranslationMissing { translation } => Some(translation.as_str()),
            _ => None,
        }
    }
}

impl From<std::io::Error> for FaithError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<rusqlite::Error> for FaithError {
    fn from(value: rusqlite::Error) -> Self {
        Self::DataMissing(value.to_string())
    }
}

impl From<serde_json::Error> for FaithError {
    fn from(value: serde_json::Error) -> Self {
        Self::Io(format!("json: {value}"))
    }
}

pub type Result<T> = std::result::Result<T, FaithError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_serializes_to_stable_string() {
        let s = serde_json::to_string(&ErrorCode::RefParse).unwrap();
        assert_eq!(s, "\"E_REF_PARSE\"");
    }

    #[test]
    fn error_code_strings_match_spec() {
        assert_eq!(ErrorCode::RefParse.as_str(), "E_REF_PARSE");
        assert_eq!(ErrorCode::NotFound.as_str(), "E_NOT_FOUND");
        assert_eq!(
            ErrorCode::TranslationMissing.as_str(),
            "E_TRANSLATION_MISSING"
        );
        assert_eq!(ErrorCode::DataMissing.as_str(), "E_DATA_MISSING");
        assert_eq!(ErrorCode::Io.as_str(), "E_IO");
    }
}
