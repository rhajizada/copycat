use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::language::detect_language;

/// Generates a Markdown string by reading the contents of the given files
/// and formatting each one within a Markdown heading and optional code fence.
///
/// # Arguments
///
/// * `root` - The base path used to determine relative file paths in the Markdown output.
/// * `files` - A slice of file paths to process and include in the generated Markdown.
///
/// # Returns
///
/// A `Result<String>` containing the full Markdown representation of all files,
/// or an error if file reading fails.
pub fn generate_markdown(root: &Path, files: &[PathBuf]) -> Result<String> {
    let mut output = String::new();

    for file in files {
        let rel_path = file.strip_prefix(root).unwrap_or(file.as_path());
        let contents = fs::read_to_string(file)?;
        let language = detect_language(file);

        // Add a section heading with the relative path
        output.push_str(&format!("### `{}`\n\n", rel_path.display()));

        // For Markdown files, embed directly; otherwise, fence code blocks
        if language == "markdown" {
            output.push_str(&contents);
            output.push_str("\n\n");
        } else {
            output.push_str(&format!("```{}\n{}\n```\n\n", language, contents));
        }
    }

    Ok(output)
}
