use anyhow::Result;
use ignore::overrides::{Override, OverrideBuilder};
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Collect files from `path`, honoring .gitignore and
/// additional user-defined exclude patterns via `excludes`.
///
/// We'll prefix each exclude pattern with `!`, which means “exclude” in override logic.
pub fn collect_files(path: PathBuf, excludes: &[String], sort: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if path.is_file() {
        files.push(path);
        return Ok(files);
    }

    // Build override rules with forced exclusion
    let overrides = build_override(excludes, &path)?;

    let walker = WalkBuilder::new(&path)
        .standard_filters(true)
        .hidden(false)
        .follow_links(true)
        .ignore(true)
        .git_ignore(true)
        .git_exclude(false)
        .git_global(true)
        .overrides(overrides)
        .build();

    for result in walker {
        let entry = result?;
        if matches!(entry.file_type(), Some(ft) if ft.is_file()) {
            files.push(entry.path().to_path_buf());
        }
    }

    if sort {
        files.sort();
    }

    Ok(files)
}

/// Builds an `Override` set from the given CLI exclude patterns.
/// By prefixing each pattern with `!`, we tell the override to exclude it.
fn build_override(excludes: &[String], root: &Path) -> Result<Override> {
    let mut builder = OverrideBuilder::new(root);
    builder.add("!.git")?;
    for pattern in excludes {
        builder.add(&format!("!{}", pattern))?;
    }
    Ok(builder.build()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn collects_single_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("hello.txt");
        File::create(&file_path).unwrap();

        let files = collect_files(file_path.clone(), &[], false).unwrap();
        assert_eq!(files, vec![file_path]);
    }

    #[test]
    fn collects_files_from_directory() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.rs");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let mut files = collect_files(dir.path().to_path_buf(), &[], false).unwrap();
        files.sort();
        let mut expected = vec![file1, file2];
        expected.sort();

        assert_eq!(files, expected);
    }

    #[test]
    fn respects_exclude_patterns() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("ignore.me");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let excludes = vec!["ignore.me".to_string()];
        let files = collect_files(dir.path().to_path_buf(), &excludes, false).unwrap();

        assert_eq!(files, vec![file1]);
    }

    #[test]
    fn respects_gitignore() {
        let dir = tempdir().unwrap();

        fs::create_dir(dir.path().join(".git")).unwrap();

        let gitignore_path = dir.path().join(".gitignore");
        let mut gitignore = File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "ignored.rs").unwrap();

        let ignored_file = dir.path().join("ignored.rs");
        let kept_file = dir.path().join("main.rs");
        File::create(&ignored_file).unwrap();
        File::create(&kept_file).unwrap();

        let mut files = collect_files(dir.path().to_path_buf(), &[], false).unwrap();
        files.sort();
        assert_eq!(files, vec![gitignore_path.clone(), kept_file]);
    }

    #[test]
    fn respects_combined_gitignore_and_excludes() {
        let dir = tempdir().unwrap();

        fs::create_dir(dir.path().join(".git")).unwrap();

        let gitignore_path = dir.path().join(".gitignore");
        let mut gitignore = File::create(&gitignore_path).unwrap();
        writeln!(gitignore, "ignored_by_gitignore.rs").unwrap();

        let file_ignored_by_gitignore = dir.path().join("ignored_by_gitignore.rs");
        let file_ignored_by_exclude = dir.path().join("excluded_by_flag.txt");
        let file_kept = dir.path().join("main.rs");

        File::create(&file_ignored_by_gitignore).unwrap();
        File::create(&file_ignored_by_exclude).unwrap();
        File::create(&file_kept).unwrap();

        let excludes = vec!["excluded_by_flag.txt".to_string()];
        let mut files = collect_files(dir.path().to_path_buf(), &excludes, false).unwrap();
        files.sort();

        assert_eq!(files, vec![gitignore_path.clone(), file_kept]);
    }

    #[test]
    fn excludes_files_with_glob_pattern() {
        let dir = tempdir().unwrap();

        let logs_dir = dir.path().join("logs");
        fs::create_dir_all(&logs_dir).unwrap();

        let top_log = dir.path().join("top_level.log");
        let nested_log = logs_dir.join("error.log");
        let allowed_file = dir.path().join("main.rs");

        File::create(&top_log).unwrap();
        File::create(&nested_log).unwrap();
        File::create(&allowed_file).unwrap();

        let excludes = vec!["**/*.log".to_string()];
        let files = collect_files(dir.path().to_path_buf(), &excludes, true).unwrap();

        assert_eq!(files, vec![allowed_file]);
    }

    #[test]
    fn sorts_files_when_requested() {
        let dir = tempdir().unwrap();
        let file_a = dir.path().join("z.rs");
        let file_b = dir.path().join("a.rs");
        File::create(&file_a).unwrap();
        File::create(&file_b).unwrap();

        let files = collect_files(dir.path().to_path_buf(), &[], true).unwrap();

        assert_eq!(files, vec![file_b, file_a]);
    }

    #[test]
    fn collects_hidden_files_and_dirs() {
        let dir = tempdir().unwrap();
        let hidden_file = dir.path().join(".secret");
        let hidden_dir = dir.path().join(".hidden");
        let hidden_dir_file = hidden_dir.join("inside.txt");
        fs::create_dir(&hidden_dir).unwrap();
        File::create(&hidden_file).unwrap();
        File::create(&hidden_dir_file).unwrap();

        let mut files = collect_files(dir.path().to_path_buf(), &[], true).unwrap();
        files.sort();
        let expected = vec![hidden_dir_file.clone(), hidden_file.clone()];
        assert_eq!(files, expected);
    }

    #[test]
    fn skips_git_directory_contents() {
        let dir = tempdir().unwrap();
        let git_dir = dir.path().join(".git");
        let git_file = git_dir.join("config");
        fs::create_dir_all(&git_dir).unwrap();
        File::create(&git_file).unwrap();
        let file = dir.path().join("main.rs");
        File::create(&file).unwrap();

        let files = collect_files(dir.path().to_path_buf(), &[], true).unwrap();
        assert_eq!(files, vec![file]);
    }
}
