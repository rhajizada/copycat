use crate::files::collect_files;
use anyhow::{anyhow, Result};
use std::{ffi::OsStr, path::PathBuf};

/// A node in the in-memory directory tree.
struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    /// Create a new node with the given name.
    pub fn new(name: String) -> Self {
        TreeNode {
            name,
            children: Vec::new(),
        }
    }

    /// Insert a sequence of path components into this node.
    pub fn insert(&mut self, comps: &[&OsStr]) {
        if comps.is_empty() {
            return;
        }
        let part = comps[0].to_string_lossy().into_owned();
        // Find existing child or create a new one
        let child = match self.children.iter_mut().find(|c| c.name == part) {
            Some(c) => c,
            None => {
                self.children.push(TreeNode::new(part.clone()));
                self.children.last_mut().unwrap()
            }
        };
        // Recurse on the remaining components
        child.insert(&comps[1..]);
    }

    /// Recursively sort this node’s children by name.
    pub fn sort(&mut self) {
        self.children.sort_by(|a, b| a.name.cmp(&b.name));
        for child in &mut self.children {
            child.sort();
        }
    }

    /// Render this node and its subtree as an ASCII tree.
    ///
    /// `prefix` is the accumulated indent, `is_last` whether this node
    /// is the last child at its level (to pick └ vs ├).
    pub fn fmt(&self, prefix: &str, is_last: bool) -> String {
        // choose the branch pointer
        let pointer = if is_last { "└── " } else { "├── " };

        // this node’s own line
        let mut out = format!("{}{}{}\n", prefix, pointer, self.name);

        // prepare prefix for children
        let new_prefix = if is_last {
            format!("{}    ", prefix) // pad spaces under a └──
        } else {
            format!("{}│   ", prefix) // keep the │ going
        };

        // render each child
        let last_idx = self.children.len().saturating_sub(1);
        for (i, child) in self.children.iter().enumerate() {
            let last = i == last_idx;
            out.push_str(&child.fmt(&new_prefix, last));
        }

        out
    }
}

/// Build a directory-tree string of `path`, honoring ignores & excludes.
pub fn collect_tree(path: PathBuf, excludes: &[String], sort: bool) -> Result<String> {
    let files = collect_files(path.clone(), excludes, sort)
        .map_err(|e| anyhow!("failed to collect files: {}", e))?;

    let root_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".")
        .to_string();
    let mut root = TreeNode::new(root_name.clone());

    for file in files {
        if let Ok(rel) = file.strip_prefix(&path) {
            let comps: Vec<&OsStr> = rel.components().map(|c| c.as_os_str()).collect();
            root.insert(&comps);
        }
    }

    if sort {
        root.sort();
    }

    let mut output = format!("{}\n", root_name);
    let last_idx = root.children.len().saturating_sub(1);
    for (i, child) in root.children.into_iter().enumerate() {
        let last = i == last_idx;
        output.push_str(&child.fmt("", last));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::collect_tree;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_empty_dir() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let tree = collect_tree(root.clone(), &[], true).unwrap();
        let root_name = root.file_name().unwrap().to_str().unwrap();
        assert_eq!(tree, format!("{}\n", root_name));
    }

    #[test]
    fn test_single_file() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let file = root.join("foo.txt");
        File::create(&file).unwrap();

        let tree = collect_tree(root.clone(), &[], true).unwrap();
        let root_name = root.file_name().unwrap().to_str().unwrap();
        let expected = format!("{}\n└── foo.txt\n", root_name);
        assert_eq!(tree, expected);
    }

    #[test]
    fn test_nested_dirs_sorted() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();

        // create nested structure
        fs::create_dir_all(root.join("a/b")).unwrap();
        File::create(root.join("a/b/file2.rs")).unwrap();
        File::create(root.join("a/file1.rs")).unwrap();

        let tree = collect_tree(root.clone(), &[], true).unwrap();
        let root_name = root.file_name().unwrap().to_str().unwrap();

        let expected = format!(
            "{root}\n\
└── a\n\
{i}├── b\n\
{i}│   └── file2.rs\n\
{i}└── file1.rs\n",
            root = root_name,
            i = "    "
        );
        assert_eq!(tree, expected);
    }

    #[test]
    fn test_excludes() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();

        File::create(root.join("keep.rs")).unwrap();
        File::create(root.join("ignore.rs")).unwrap();

        let tree = collect_tree(root.clone(), &["ignore.rs".into()], true).unwrap();
        let root_name = root.file_name().unwrap().to_str().unwrap();

        let expected = format!("{r}\n└── keep.rs\n", r = root_name);
        assert_eq!(tree, expected);
    }
}
