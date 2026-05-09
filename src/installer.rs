//! HelloAO installer: downloads `complete.json`, parses verses, persists.

use serde::Deserialize;

use crate::error::{FaithError, Result};
use crate::store::{Store, StoredTranslation};
use crate::translations::TranslationDef;

#[derive(Debug, Deserialize)]
struct CompleteRoot {
    books: Vec<RawBook>,
}

#[derive(Debug, Deserialize)]
struct RawBook {
    id: String,
    chapters: Vec<RawChapterWrap>,
}

#[derive(Debug, Deserialize)]
struct RawChapterWrap {
    chapter: RawChapter,
}

#[derive(Debug, Deserialize)]
struct RawChapter {
    number: u16,
    content: Vec<serde_json::Value>,
}

pub fn install(store: &mut Store, def: &TranslationDef) -> Result<u32> {
    let body = http_get(def.source_url)?;
    let root: CompleteRoot = serde_json::from_str(&body)
        .map_err(|e| FaithError::Io(format!("parse complete.json: {e}")))?;
    let verses = extract_verses(&root)?;
    let count = verses.len() as u32;

    let now = current_iso8601();
    let stored = StoredTranslation {
        id: def.alias.to_string(),
        name: def.name.to_string(),
        english_name: def.english_name.to_string(),
        language: def.language.to_string(),
        direction: def.direction.to_string(),
        license: def.license.to_string(),
        source_url: def.source_url.to_string(),
        installed_at: now,
        books: 0,
        verses: 0,
    };
    store.upsert_translation(&stored)?;
    store.replace_verses(def.alias, &verses)?;
    Ok(count)
}

fn extract_verses(root: &CompleteRoot) -> Result<Vec<(String, u16, u16, String)>> {
    let mut out = Vec::with_capacity(31_500);
    for book in &root.books {
        let book_id = book.id.to_ascii_uppercase();
        for ch in &book.chapters {
            let chapter_no = ch.chapter.number;
            for item in &ch.chapter.content {
                if item.get("type").and_then(|v| v.as_str()) != Some("verse") {
                    continue;
                }
                let Some(verse_no) = item.get("number").and_then(|v| v.as_u64()) else {
                    continue;
                };
                let Some(content_arr) = item.get("content").and_then(|v| v.as_array()) else {
                    continue;
                };
                let text = collect_verse_text(content_arr);
                if !text.is_empty() {
                    out.push((book_id.clone(), chapter_no, verse_no as u16, text));
                }
            }
        }
    }
    if out.is_empty() {
        return Err(FaithError::Io("no verses extracted".into()));
    }
    Ok(out)
}

fn collect_verse_text(content: &[serde_json::Value]) -> String {
    let mut buf = String::new();
    for piece in content {
        let segment: Option<&str> = if let Some(s) = piece.as_str() {
            Some(s)
        } else if let Some(obj) = piece.as_object() {
            obj.get("text").and_then(|v| v.as_str())
        } else {
            None
        };
        let Some(s) = segment else { continue };
        let trimmed = s.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !buf.is_empty() && !buf.ends_with(' ') {
            buf.push(' ');
        }
        buf.push_str(trimmed);
    }
    while let Some(stripped) = buf.strip_prefix('¶') {
        buf = stripped.trim_start().to_string();
    }
    buf.trim().to_string()
}

fn current_iso8601() -> String {
    let now = time::OffsetDateTime::now_utc();
    now.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

fn http_get(url: &str) -> Result<String> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(60))
        .call()
        .map_err(|e| FaithError::Io(format!("HTTP {url}: {e}")))?;
    let body = resp
        .into_string()
        .map_err(|e| FaithError::Io(format!("read body {url}: {e}")))?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_verse_text_skipping_notes() {
        let content = vec![
            serde_json::json!("In the beginning"),
            serde_json::json!({"noteId": 0}),
            serde_json::json!("God created."),
        ];
        assert_eq!(
            collect_verse_text(&content),
            "In the beginning God created."
        );
    }

    #[test]
    fn strips_leading_pilcrow() {
        let content = vec![serde_json::json!("¶ And God said.")];
        assert_eq!(collect_verse_text(&content), "And God said.");
    }

    #[test]
    fn collects_object_text_segments() {
        // HelloAO formats wordsOfJesus and other styled spans as objects.
        let content = vec![
            serde_json::json!("Jesus answered,"),
            serde_json::json!({"text":"Verily I say unto thee.","wordsOfJesus":true}),
        ];
        assert_eq!(
            collect_verse_text(&content),
            "Jesus answered, Verily I say unto thee."
        );
    }

    #[test]
    fn collects_object_only_verse_with_pilcrow() {
        // John 3:16 in HelloAO KJV is a single object with leading pilcrow.
        let content = vec![serde_json::json!({
            "text":"¶ For God so loved the world.",
            "wordsOfJesus":true,
        })];
        assert_eq!(collect_verse_text(&content), "For God so loved the world.");
    }

    #[test]
    fn ignores_objects_without_text_field() {
        let content = vec![
            serde_json::json!("In the beginning"),
            serde_json::json!({"noteId": 0}),
            serde_json::json!("God created."),
        ];
        assert_eq!(
            collect_verse_text(&content),
            "In the beginning God created."
        );
    }
}
