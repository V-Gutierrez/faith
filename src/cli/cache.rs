//! Cache management subcommand.
//!
//! Provides: `faith cache size`, `faith cache clear --confirm`, `faith cache path`

use std::fs;
use std::io;
use std::path::Path;

use crate::error::FaithError;
use crate::schema::CacheStatsOut;

/// Run cache subcommand.
pub fn run(
    subcommand: &str,
    confirm: bool,
    out: &mut dyn std::io::Write,
) -> Result<i32, FaithError> {
    match subcommand {
        "size" => size(out),
        "clear" => clear(confirm, out),
        "path" => path(out),
        _ => Err(FaithError::RefParse {
            input: format!(
                "unknown cache subcommand '{}'; use: size, clear, path",
                subcommand
            ),
        }),
    }
}

/// Get cache and DB sizes.
fn size(out: &mut dyn std::io::Write) -> Result<i32, FaithError> {
    let path = crate::store::default_db_path()?;
    let base = path
        .parent()
        .ok_or_else(|| FaithError::Io("no parent dir".into()))?;

    let db_bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

    let cache_dir = base.join("cache");
    let cache_bytes = dir_size(&cache_dir).unwrap_or(0);

    let manifest_file = base.join("manifest.json");
    let manifest_bytes = fs::metadata(&manifest_file).map(|m| m.len()).unwrap_or(0);

    let total_bytes = db_bytes + cache_bytes + manifest_bytes;

    let stats = CacheStatsOut {
        schema: crate::schema::SCHEMA_VERSION,
        kind: "cache_stats",
        db_bytes,
        cache_bytes,
        manifest_bytes,
        total_bytes,
        path: base.display().to_string(),
    };

    serde_json::to_writer(&mut *out, &stats).map_err(|e| FaithError::Io(e.to_string()))?;
    writeln!(out).map_err(|e| FaithError::Io(e.to_string()))?;

    Ok(0)
}

/// Clear cache directory.
fn clear(confirm: bool, out: &mut dyn std::io::Write) -> Result<i32, FaithError> {
    if !confirm {
        return Err(FaithError::RefParse {
            input: "use --confirm to delete cache".into(),
        });
    }

    let path = crate::store::default_db_path()?;
    let base = path
        .parent()
        .ok_or_else(|| FaithError::Io("no parent dir".into()))?;
    let cache_dir = base.join("cache");

    if !cache_dir.exists() {
        // Idempotent: already cleared
        let msg = format!(
            "{{\"schema\":\"{}\",\"kind\":\"message\",\"message\":\"Cache dir not found (already cleared)\"}}",
            crate::schema::SCHEMA_VERSION
        );
        writeln!(out, "{}", msg).map_err(|e| FaithError::Io(e.to_string()))?;
        return Ok(0);
    }

    let before = dir_size(&cache_dir).unwrap_or(0);
    fs::remove_dir_all(&cache_dir)
        .map_err(|e| FaithError::Io(format!("failed to remove cache dir: {}", e)))?;

    let freed_mb = before as f64 / 1_000_000.0;
    let msg = format!(
        "{{\"schema\":\"{}\",\"kind\":\"message\",\"message\":\"Cleared {} (freed {:.1}MB)\"}}",
        crate::schema::SCHEMA_VERSION,
        cache_dir.display(),
        freed_mb
    );
    writeln!(out, "{}", msg).map_err(|e| FaithError::Io(e.to_string()))?;

    Ok(0)
}

/// Print cache path.
fn path(out: &mut dyn std::io::Write) -> Result<i32, FaithError> {
    let path = crate::store::default_db_path()?;
    let base = path
        .parent()
        .ok_or_else(|| FaithError::Io("no parent dir".into()))?;
    writeln!(out, "{}", base.display()).map_err(|e| FaithError::Io(e.to_string()))?;
    Ok(0)
}

/// Recursively compute directory size in bytes.
fn dir_size(path: &Path) -> io::Result<u64> {
    let mut total = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            total += if meta.is_dir() {
                dir_size(&entry.path())?
            } else {
                meta.len()
            };
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn cache_clear_without_confirm_returns_error() {
        let mut buf = Cursor::new(Vec::<u8>::new());
        let result = clear(false, &mut buf);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("--confirm"));
    }

    #[test]
    fn dir_size_empty_dir_returns_zero() {
        let d = tempfile::tempdir().unwrap();
        let size = dir_size(d.path()).unwrap();
        assert_eq!(size, 0);
    }

    #[test]
    fn dir_size_with_file_returns_bytes() {
        let d = tempfile::tempdir().unwrap();
        fs::write(d.path().join("test.txt"), "hello").unwrap();
        let size = dir_size(d.path()).unwrap();
        assert!(size > 0);
    }
}
