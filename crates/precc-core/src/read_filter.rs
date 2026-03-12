//! Read tool optimization: binary file blocking, smart limit injection,
//! and duplicate read suppression.

use std::path::Path;

/// Known binary file extensions that should be blocked from Read.
const BINARY_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "svg", "webp", "tiff", "tif", "wasm", "o", "so",
    "a", "exe", "dll", "dylib", "lib", "zip", "tar", "gz", "bz2", "xz", "7z", "rar", "zst", "bin",
    "dat", "class", "pyc", "pyo", "ttf", "otf", "woff", "woff2", "eot", "mp3", "mp4", "avi", "mov",
    "mkv", "flac", "wav",
];

/// Threshold (in lines) above which we suggest a limit.
const LARGE_FILE_THRESHOLD: u64 = 2000;

/// Default limit to inject for large files with no limit set.
const DEFAULT_SUGGESTED_LIMIT: u64 = 500;

/// Check if a file path has a known binary extension.
pub fn is_binary_extension(path: &str) -> bool {
    let path_lower = path.to_lowercase();
    if let Some(ext) = Path::new(&path_lower).extension().and_then(|e| e.to_str()) {
        BINARY_EXTENSIONS.contains(&ext)
    } else {
        false
    }
}

/// Suggest a limit for reading a file, if the file is large and no limit is set.
/// Returns `None` if a limit is already set or the file is small enough.
pub fn suggest_limit(path: &str, current_limit: Option<u64>) -> Option<u64> {
    // If a limit is already set, don't override it
    if current_limit.is_some() {
        return None;
    }

    // Check file size in lines (approximate via file size / avg line length)
    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return None, // File doesn't exist yet or inaccessible — pass through
    };

    if !metadata.is_file() {
        return None;
    }

    // Rough heuristic: avg 60 bytes per line
    let estimated_lines = metadata.len() / 60;
    if estimated_lines > LARGE_FILE_THRESHOLD {
        Some(DEFAULT_SUGGESTED_LIMIT)
    } else {
        None
    }
}

/// Check if this file was recently read (within the last 60 seconds).
/// Returns `true` if a duplicate read is detected.
pub fn check_recent_read(
    data_dir: &Path,
    file_path: &str,
    offset: Option<u64>,
    limit: Option<u64>,
) -> bool {
    let cache_path = data_dir.join("read_cache.txt");
    let key = format!(
        "{}:{}:{}",
        file_path,
        offset.unwrap_or(0),
        limit.unwrap_or(0)
    );

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Read existing entries, check for recent match
    if let Ok(content) = std::fs::read_to_string(&cache_path) {
        for line in content.lines().rev().take(100) {
            // Format: timestamp:key
            if let Some((ts_str, entry_key)) = line.split_once(':') {
                if let Ok(ts) = ts_str.parse::<u64>() {
                    if now.saturating_sub(ts) <= 60 && entry_key == key {
                        return true;
                    }
                }
            }
        }
    }

    // Append this read to the cache
    let entry = format!("{}:{}\n", now, key);
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&cache_path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, entry.as_bytes()));

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_extension_detected() {
        assert!(is_binary_extension("/tmp/image.png"));
        assert!(is_binary_extension("/tmp/file.PNG")); // case insensitive
        assert!(is_binary_extension("test.wasm"));
        assert!(is_binary_extension("libfoo.so"));
        assert!(is_binary_extension("archive.tar.gz"));
    }

    #[test]
    fn non_binary_extension_passes() {
        assert!(!is_binary_extension("/tmp/main.rs"));
        assert!(!is_binary_extension("/tmp/Cargo.toml"));
        assert!(!is_binary_extension("/tmp/README.md"));
        assert!(!is_binary_extension("/tmp/config.json"));
    }

    #[test]
    fn no_extension_passes() {
        assert!(!is_binary_extension("/tmp/Makefile"));
        assert!(!is_binary_extension("Dockerfile"));
    }

    #[test]
    fn suggest_limit_with_existing_limit() {
        assert_eq!(suggest_limit("/tmp/test.rs", Some(100)), None);
    }

    #[test]
    fn suggest_limit_nonexistent_file() {
        assert_eq!(suggest_limit("/nonexistent/path/foo.rs", None), None);
    }

    #[test]
    fn duplicate_read_detection() {
        let tmp = tempfile::tempdir().unwrap();
        let data_dir = tmp.path();

        // First read: not a duplicate
        assert!(!check_recent_read(data_dir, "/tmp/foo.rs", None, None));

        // Second read of same file: duplicate
        assert!(check_recent_read(data_dir, "/tmp/foo.rs", None, None));

        // Different file: not a duplicate
        assert!(!check_recent_read(data_dir, "/tmp/bar.rs", None, None));

        // Same file, different offset: not a duplicate
        assert!(!check_recent_read(data_dir, "/tmp/foo.rs", Some(100), None));
    }
}
