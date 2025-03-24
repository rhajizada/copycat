use anyhow::Result;
use ignore::WalkBuilder;
use std::path::PathBuf;

pub fn collect_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if path.is_file() {
        files.push(path);
        return Ok(files);
    }

    let walker = WalkBuilder::new(&path).standard_filters(true).build();

    for result in walker {
        let entry = result?;
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            files.push(entry.path().to_path_buf());
        }
    }

    Ok(files)
}
