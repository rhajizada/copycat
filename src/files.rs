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
        .follow_links(false)
        .ignore(true)
        .git_ignore(true)
        .git_exclude(true)
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
    for pattern in excludes {
        builder.add(&format!("!{}", pattern))?;
    }
    Ok(builder.build()?)
}
