use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct LanguageData {
    pub extensions: Vec<String>,
    pub filenames: Vec<String>,
}

// We'll embed our curated languages.json at compile time
static LANGUAGES_JSON: &str = include_str!("../assets/languages.json");

// Use a HashMap<String, String> so we store owned data in a static
static EXT_TO_TAG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // parse a map: { "tag_name": { "extensions": [], "filenames": [] }, ... }
    let parsed: HashMap<String, LanguageData> =
        serde_json::from_str(LANGUAGES_JSON).expect("Invalid languages.json");

    let mut map = HashMap::new();

    // Each KEY in `parsed` is the `tag` (e.g., 'python', 'rust', etc.)
    for (tag, data) in parsed {
        // Insert each extension => tag
        for ext in data.extensions {
            let ext_str = ext.trim_start_matches('.').to_string();
            map.insert(ext_str, tag.clone());
        }
        // Insert each filename => tag
        for name in data.filenames {
            map.insert(name, tag.clone());
        }
    }

    map
});

/// Detect the code block tag (e.g. "rust", "python", "markdown")
pub fn detect_language(path: &Path) -> &str {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        if let Some(tag) = EXT_TO_TAG.get(ext) {
            return tag;
        }
    }

    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
        if let Some(tag) = EXT_TO_TAG.get(name) {
            return tag;
        }
    }

    // Fallback
    "text"
}
