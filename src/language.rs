use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// Data describing the file extensions and filenames for a given syntax tag.
/// For instance: tag = "rust", extensions = [".rs"]
#[derive(Debug, Deserialize)]
pub struct LanguageData {
    /// File extensions (e.g. ".rs", ".py") that map to a particular code block tag.
    pub extensions: Vec<String>,
    /// Exact filenames (e.g. "Dockerfile", "Makefile") that map to a particular code block tag.
    pub filenames: Vec<String>,
}

/// A compiled JSON file mapping code block tags to their corresponding
/// [`LanguageData`] (extensions & filenames).
static LANGUAGES_JSON: &str = include_str!("../assets/languages.json");

/// A static map from extension/filename -> code block tag,
/// used by [`detect_language`] to figure out how to fence code blocks.
static EXT_TO_TAG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let parsed: HashMap<String, LanguageData> =
        serde_json::from_str(LANGUAGES_JSON).expect("invalid languages.json");

    let mut map = HashMap::new();

    for (tag, data) in parsed {
        for ext in data.extensions {
            let ext_str = ext.trim_start_matches('.').to_string();
            map.insert(ext_str, tag.clone());
        }
        for name in data.filenames {
            map.insert(name, tag.clone());
        }
    }

    map
});

/// Detects the code block tag for the given `path` based on its file extension
/// or full filename, using the mapping from [`LANGUAGES_JSON`].
///
/// If no known mapping is found, it returns `"text"` as a fallback.
pub fn detect_language(path: &Path) -> &str {
    match (
        path.extension().and_then(|s| s.to_str()),
        path.file_name().and_then(|s| s.to_str()),
    ) {
        (Some(ext), _) if EXT_TO_TAG.contains_key(ext) => EXT_TO_TAG.get(ext).unwrap(),
        (_, Some(name)) if EXT_TO_TAG.contains_key(name) => EXT_TO_TAG.get(name).unwrap(),
        _ => "text",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_detect_rust_extension() {
        let path = Path::new("main.rs");
        assert_eq!(
            detect_language(path),
            "rust",
            "`.rs` files should return `rust`"
        );
    }

    #[test]
    fn test_detect_python_extension() {
        let path = Path::new("script.py");
        assert_eq!(
            detect_language(path),
            "python",
            "`.py` files should return `python`"
        );
    }

    #[test]
    fn test_detect_markdown() {
        let path = Path::new("README");
        assert_eq!(
            detect_language(path),
            "markdown",
            "files named `README.md` should return `markdown`"
        );
    }

    #[test]
    fn test_detect_unknown() {
        let path = Path::new("some.unknownext");
        assert_eq!(
            detect_language(path),
            "text",
            "unknown extension should fall back to `text`"
        );
    }
}
