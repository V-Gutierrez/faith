//! SQLite-backed verse store.
//!
//! Schema lives in `docs/SPEC.md`. FTS5 virtual table is created but unused
//! in v0.1; populated by `installer` for v0.2 search.

use std::path::{Path, PathBuf};

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{FaithError, Result};

pub struct Store {
    conn: Connection,
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct StoredTranslation {
    pub id: String,
    pub name: String,
    pub english_name: String,
    pub language: String,
    pub direction: String,
    pub license: String,
    pub source_url: String,
    pub installed_at: String,
    pub books: u16,
    pub verses: u32,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let conn = Connection::open(path)?;
        let s = Self {
            conn,
            path: path.to_path_buf(),
        };
        s.init_schema()?;
        Ok(s)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS translations (
              id TEXT PRIMARY KEY,
              name TEXT NOT NULL,
              english_name TEXT,
              language TEXT NOT NULL,
              direction TEXT NOT NULL,
              license TEXT,
              source_url TEXT,
              installed_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS verses (
              translation TEXT NOT NULL,
              book TEXT NOT NULL,
              chapter INTEGER NOT NULL,
              verse INTEGER NOT NULL,
              text TEXT NOT NULL,
              PRIMARY KEY (translation, book, chapter, verse),
              FOREIGN KEY (translation) REFERENCES translations(id)
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS verses_fts USING fts5(
              text,
              content='verses',
              content_rowid='rowid',
              tokenize='unicode61'
            );
            "#,
        )?;
        Ok(())
    }

    pub fn upsert_translation(&mut self, t: &StoredTranslation) -> Result<()> {
        self.conn.execute(
            r#"INSERT INTO translations
                 (id, name, english_name, language, direction, license, source_url, installed_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
               ON CONFLICT(id) DO UPDATE SET
                 name=excluded.name,
                 english_name=excluded.english_name,
                 language=excluded.language,
                 direction=excluded.direction,
                 license=excluded.license,
                 source_url=excluded.source_url,
                 installed_at=excluded.installed_at"#,
            params![
                t.id,
                t.name,
                t.english_name,
                t.language,
                t.direction,
                t.license,
                t.source_url,
                t.installed_at,
            ],
        )?;
        Ok(())
    }

    pub fn replace_verses(
        &mut self,
        translation: &str,
        verses: &[(String, u16, u16, String)],
    ) -> Result<()> {
        let tx = self.conn.transaction()?;
        // Delete old FTS entries for this translation by matching rowids
        tx.execute_batch(
            "DELETE FROM verses_fts WHERE rowid IN (
               SELECT rowid FROM verses WHERE translation = ?1
             )"
            .replace("?1", &format!("'{}'", translation.replace('\'', "''")))
            .as_str(),
        )?;
        tx.execute(
            "DELETE FROM verses WHERE translation = ?1",
            params![translation],
        )?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO verses (translation, book, chapter, verse, text) VALUES (?1,?2,?3,?4,?5)",
            )?;
            let mut fts_stmt = tx.prepare(
                "INSERT INTO verses_fts (rowid, text) VALUES (last_insert_rowid(), ?1)",
            )?;
            for (book, chapter, verse, text) in verses {
                stmt.execute(params![translation, book, chapter, verse, text])?;
                fts_stmt.execute(params![text])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_verse(
        &self,
        translation: &str,
        book: &str,
        chapter: u16,
        verse: u16,
    ) -> Result<Option<String>> {
        let v = self
            .conn
            .query_row(
                "SELECT text FROM verses WHERE translation=?1 AND book=?2 AND chapter=?3 AND verse=?4",
                params![translation, book, chapter, verse],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(v)
    }

    pub fn get_chapter(
        &self,
        translation: &str,
        book: &str,
        chapter: u16,
    ) -> Result<Vec<(u16, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT verse, text FROM verses WHERE translation=?1 AND book=?2 AND chapter=?3 ORDER BY verse",
        )?;
        let rows = stmt
            .query_map(params![translation, book, chapter], |row| {
                Ok((row.get::<_, u16>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn get_range(
        &self,
        translation: &str,
        book: &str,
        start_chapter: u16,
        start_verse: u16,
        end_chapter: u16,
        end_verse: u16,
    ) -> Result<Vec<(u16, u16, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT chapter, verse, text FROM verses
             WHERE translation = ?1 AND book = ?2
               AND ( (chapter = ?3 AND chapter = ?5 AND verse >= ?4 AND verse <= ?6)
                  OR (chapter = ?3 AND ?3 < ?5 AND verse >= ?4)
                  OR (chapter > ?3 AND chapter < ?5)
                  OR (chapter = ?5 AND ?3 < ?5 AND verse <= ?6) )
             ORDER BY chapter, verse",
        )?;
        let rows = stmt
            .query_map(
                params![
                    translation,
                    book,
                    start_chapter,
                    start_verse,
                    end_chapter,
                    end_verse,
                ],
                |row| {
                    Ok((
                        row.get::<_, u16>(0)?,
                        row.get::<_, u16>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn list_translations(&self) -> Result<Vec<StoredTranslation>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.name, t.english_name, t.language, t.direction, t.license, t.source_url, t.installed_at,
                    (SELECT COUNT(DISTINCT book) FROM verses v WHERE v.translation=t.id) AS books,
                    (SELECT COUNT(*) FROM verses v WHERE v.translation=t.id) AS verses
             FROM translations t
             ORDER BY t.id",
        )?;
        let rows = stmt
            .query_map([], |row| {
                Ok(StoredTranslation {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    english_name: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                    language: row.get(3)?,
                    direction: row.get(4)?,
                    license: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                    source_url: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                    installed_at: row.get(7)?,
                    books: row.get::<_, i64>(8)? as u16,
                    verses: row.get::<_, i64>(9)? as u32,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    pub fn translation_exists(&self, id: &str) -> Result<bool> {
        let n: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM translations WHERE id=?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(n > 0)
    }

    pub fn require_translation(&self, id: &str) -> Result<()> {
        if self.translation_exists(id)? {
            Ok(())
        } else {
            Err(FaithError::TranslationMissing {
                translation: id.to_string(),
            })
        }
    }

    /// Count verses for a single book within an installed translation.
    ///
    /// Translation must exist; returns `0` if the book is absent (the caller
    /// decides how to surface that — `info` reports `0`, others may treat as
    /// not-found).
    pub fn book_verse_count(&self, translation: &str, book: &str) -> Result<u32> {
        self.require_translation(translation)?;
        let n: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM verses WHERE translation=?1 AND book=?2",
            params![translation, book],
            |row| row.get(0),
        )?;
        Ok(n as u32)
    }

    /// Pick a single verse uniformly at random from a (translation, optional
    /// book filter, optional book-allowlist) population using a caller-provided
    /// `nth` index. Returns `Ok(None)` if the population is empty.
    ///
    /// `book_in` lets the caller restrict to OT/NT (passed as USFM IDs). When
    /// both `book` and `book_in` are `None` the whole translation is sampled.
    pub fn random_verse(
        &self,
        translation: &str,
        book: Option<&str>,
        book_in: Option<&[&str]>,
        nth: u64,
    ) -> Result<Option<(String, u16, u16, String)>> {
        self.require_translation(translation)?;
        let (where_extra, params_extra): (String, Vec<rusqlite::types::Value>) =
            match (book, book_in) {
                (Some(b), _) => (
                    " AND book = ?2".to_string(),
                    vec![rusqlite::types::Value::Text(b.to_string())],
                ),
                (None, Some(list)) if !list.is_empty() => {
                    let placeholders: Vec<String> =
                        (2..2 + list.len()).map(|i| format!("?{i}")).collect();
                    (
                        format!(" AND book IN ({})", placeholders.join(",")),
                        list.iter()
                            .map(|s| rusqlite::types::Value::Text((*s).to_string()))
                            .collect(),
                    )
                }
                _ => (String::new(), Vec::new()),
            };

        let count_sql = format!("SELECT COUNT(*) FROM verses WHERE translation = ?1{where_extra}");
        let mut count_params: Vec<rusqlite::types::Value> =
            vec![rusqlite::types::Value::Text(translation.to_string())];
        count_params.extend(params_extra.iter().cloned());
        let total: i64 = self.conn.query_row(
            &count_sql,
            rusqlite::params_from_iter(count_params.iter()),
            |row| row.get(0),
        )?;
        if total == 0 {
            return Ok(None);
        }

        let offset = (nth % total as u64) as i64;
        let select_sql = format!(
            "SELECT book, chapter, verse, text FROM verses
             WHERE translation = ?1{where_extra}
             ORDER BY book, chapter, verse
             LIMIT 1 OFFSET ?{}",
            2 + params_extra.len()
        );
        let mut select_params: Vec<rusqlite::types::Value> =
            vec![rusqlite::types::Value::Text(translation.to_string())];
        select_params.extend(params_extra.iter().cloned());
        select_params.push(rusqlite::types::Value::Integer(offset));

        let row = self.conn.query_row(
            &select_sql,
            rusqlite::params_from_iter(select_params.iter()),
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, u16>(1)?,
                    row.get::<_, u16>(2)?,
                    row.get::<_, String>(3)?,
                ))
            },
        )?;
        Ok(Some(row))
    }

    pub fn list_books(&self, translation: &str) -> Result<Vec<String>> {
        self.require_translation(translation)?;
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT book FROM verses WHERE translation=?1 ORDER BY book")?;
        let rows = stmt
            .query_map(params![translation], |row| row.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// `(book_id, verse_count)` pairs for an installed translation.
    ///
    /// Used by `stats` to derive OT/NT splits without a SQL join into the
    /// canonical books table (testament data lives in `crate::books`).
    pub fn translation_book_counts(&self, translation: &str) -> Result<Vec<(String, u32)>> {
        self.require_translation(translation)?;
        let mut stmt = self.conn.prepare(
            "SELECT book, COUNT(*) FROM verses WHERE translation=?1 GROUP BY book ORDER BY book",
        )?;
        let rows = stmt
            .query_map(params![translation], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as u32))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Distinct `(book, chapter)` count for an installed translation.
    pub fn translation_chapter_count(&self, translation: &str) -> Result<u32> {
        self.require_translation(translation)?;
        let n: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM (SELECT 1 FROM verses WHERE translation=?1 GROUP BY book, chapter)",
            params![translation],
            |row| row.get(0),
        )?;
        Ok(n as u32)
    }

    /// `installed_at` timestamp for a translation (ISO 8601 string).
    pub fn translation_installed_at(&self, translation: &str) -> Result<String> {
        self.require_translation(translation)?;
        let ts: String = self.conn.query_row(
            "SELECT installed_at FROM translations WHERE id=?1",
            params![translation],
            |row| row.get(0),
        )?;
        Ok(ts)
    }

    /// Full-text search across installed verses using the FTS5 index.
    ///
    /// Returns up to `limit` results ranked by BM25, optionally filtered by
    /// translation. Each result includes the translation, book, chapter, verse,
    /// matched snippet, and a relevance rank (lower = better match).
    pub fn search_verses(
        &self,
        query: &str,
        translation: Option<&str>,
        limit: u32,
    ) -> Result<Vec<SearchHit>> {
        let (where_clause, params): (String, Vec<rusqlite::types::Value>) = match translation {
            Some(tr) => (
                " AND v.translation = ?2".to_string(),
                vec![
                    rusqlite::types::Value::Text(query.to_string()),
                    rusqlite::types::Value::Text(tr.to_string()),
                ],
            ),
            None => (
                String::new(),
                vec![rusqlite::types::Value::Text(query.to_string())],
            ),
        };

        let limit_param_idx = params.len() + 1;
        let sql = format!(
            "SELECT v.translation, v.book, v.chapter, v.verse,
                    snippet(verses_fts, 0, '»', '«', '…', 24) AS snippet,
                    rank
             FROM verses_fts fts
             JOIN verses v ON v.rowid = fts.rowid
             WHERE verses_fts MATCH ?1{where_clause}
             ORDER BY rank
             LIMIT ?{limit_param_idx}"
        );

        let mut all_params = params;
        all_params.push(rusqlite::types::Value::Integer(limit as i64));

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt
            .query_map(rusqlite::params_from_iter(all_params.iter()), |row| {
                Ok(SearchHit {
                    translation: row.get(0)?,
                    book: row.get(1)?,
                    chapter: row.get(2)?,
                    verse: row.get(3)?,
                    snippet: row.get(4)?,
                    rank: row.get(5)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub translation: String,
    pub book: String,
    pub chapter: u16,
    pub verse: u16,
    pub snippet: String,
    pub rank: f64,
}

pub fn default_db_path() -> Result<PathBuf> {
    let dir = data_dir()?;
    Ok(dir.join("bible.db"))
}

pub fn data_dir() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("FAITH_DATA_DIR") {
        return Ok(PathBuf::from(p));
    }
    let proj = directories::ProjectDirs::from("dev", "faith", "faith")
        .or_else(|| directories::ProjectDirs::from("", "", "faith"))
        .ok_or_else(|| FaithError::Io("could not resolve data dir".into()))?;
    Ok(proj.data_dir().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fresh() -> (Store, tempfile::TempDir) {
        let d = tempdir().unwrap();
        let p = d.path().join("bible.db");
        let s = Store::open(&p).unwrap();
        (s, d)
    }

    #[test]
    fn open_initializes_schema() {
        let (_s, _d) = fresh();
    }

    #[test]
    fn upsert_and_lookup_round_trip() {
        let (mut s, _d) = fresh();
        s.upsert_translation(&StoredTranslation {
            id: "KJV".into(),
            name: "King James".into(),
            english_name: "King James".into(),
            language: "eng".into(),
            direction: "ltr".into(),
            license: "Public Domain".into(),
            source_url: "https://example".into(),
            installed_at: "2026-05-09T00:00:00Z".into(),
            books: 0,
            verses: 0,
        })
        .unwrap();

        s.replace_verses(
            "KJV",
            &[
                ("JHN".into(), 3, 16, "For God so loved the world.".into()),
                (
                    "JHN".into(),
                    3,
                    17,
                    "For God sent not his Son to condemn.".into(),
                ),
            ],
        )
        .unwrap();

        let v = s.get_verse("KJV", "JHN", 3, 16).unwrap();
        assert_eq!(v.as_deref(), Some("For God so loved the world."));

        let range = s.get_range("KJV", "JHN", 3, 16, 3, 17).unwrap();
        assert_eq!(range.len(), 2);

        let books = s.list_books("KJV").unwrap();
        assert_eq!(books, vec!["JHN".to_string()]);

        assert!(s.translation_exists("KJV").unwrap());
        assert!(!s.translation_exists("ONBV").unwrap());
    }
}
