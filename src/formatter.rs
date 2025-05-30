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
        let contents = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "warning: could not read file '{}': {}",
                    rel_path.display(),
                    e
                );
                continue;
            }
        };

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir; // Add dev-dependency: `tempfile = "3.5"`

    #[test]
    fn test_generate_markdown_single_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create a file
        let file_path = root.join("hello.rs");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "fn main() {{ println!(\"Hello\"); }}").unwrap();

        let files = vec![file_path.clone()];
        let md_output = generate_markdown(root, &files).unwrap();

        // Check some aspects of the output
        assert!(md_output.contains("### `hello.rs`"));
        assert!(md_output.contains("```rust"));
        assert!(md_output.contains("fn main()"));
    }

    #[test]
    fn test_generate_markdown_with_markdown_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let file_path = root.join("README.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "# Hello Markdown").unwrap();

        let files = vec![file_path.clone()];
        let md_output = generate_markdown(root, &files).unwrap();

        // Should not have triple backticks
        assert!(md_output.contains("### `README.md`"));
        assert!(md_output.contains("# Hello Markdown"));
        assert!(!md_output.contains("```markdown"));
    }

    #[test]
    fn test_generate_markdown_empty_files() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // No files
        let files: Vec<PathBuf> = vec![];
        let md_output = generate_markdown(root, &files).unwrap();

        // Should be empty, no headings
        assert!(md_output.is_empty());
    }

    #[test]
    fn test_generate_markdown_skips_unreadable_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let ok_path = root.join("ok.rs");
        let mut ok_file = File::create(&ok_path).unwrap();
        writeln!(ok_file, "fn main() {{ println!(\"ok\"); }}").unwrap();

        let missing_path = root.join("missing.rs");

        let files = vec![ok_path.clone(), missing_path.clone()];
        let md_output = generate_markdown(root, &files).unwrap();

        assert!(md_output.contains("### `ok.rs`"));
        assert!(!md_output.contains("### `missing.rs`"));
    }
}
