mod files;
mod formatter;
mod language;
mod tree;

use anyhow::Result;
use clap::{ArgAction, Parser};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::path::{Path, PathBuf};

/// Command-line arguments for the `copycat` application.
#[derive(Parser, Debug)]
#[command(
    name = "copycat",
    version,
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    /// Path to repository directory or a single file.
    path: PathBuf,

    /// One or more glob patterns for excluding files (e.g. ".gitignore", "**/*.md").
    /// Can be repeated multiple times.
    #[arg(long = "exclude", short = 'e', action = ArgAction::Append)]
    excludes: Vec<String>,

    /// Sort files alphabetically.
    #[arg(long = "sort", short = 's', action = ArgAction::SetTrue)]
    sort: bool,

    /// Only output directory tree.
    #[arg(long = "tree", short = 't', action = ArgAction::SetTrue)]
    tree: bool,

    /// Print to stdout instead of copying to clipboard.
    #[arg(long = "print", short = 'p', action = ArgAction::SetTrue)]
    print: bool,
}

/// Gather full Markdown of all collected files.
fn get_contents(path: &Path, excludes: &[String], sort: bool) -> Result<String> {
    let files = files::collect_files(path.to_path_buf(), excludes, sort)
        .map_err(|e| anyhow::anyhow!("failed to collect files: {}", e))?;

    if files.is_empty() {
        anyhow::bail!("no matching files found, nothing to copy");
    }

    let markdown = formatter::generate_markdown(path, &files)
        .map_err(|e| anyhow::anyhow!("failed to read files: {}", e))?;
    Ok(markdown)
}

/// Build an ASCII tree of all collected files & directories.
fn get_tree(path: &Path, excludes: &[String], sort: bool) -> Result<String> {
    let tree = tree::collect_tree(path.to_path_buf(), excludes, sort)
        .map_err(|e| anyhow::anyhow!("failed to build tree: {}", e))?;
    Ok(tree)
}

fn main() {
    let args = Args::parse();

    if !args.path.exists() {
        eprintln!("provided path {} does not exist", args.path.display());
        std::process::exit(1);
    }

    let output = if args.tree {
        get_tree(&args.path, &args.excludes, args.sort)
    } else {
        get_contents(&args.path, &args.excludes, args.sort)
    };

    let output = match output {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    if args.print {
        println!("{}", output);
    } else {
        let mut ctx = ClipboardContext::new().unwrap_or_else(|e| {
            eprintln!("failed to create clipboard context: {}", e);
            std::process::exit(1);
        });
        ctx.set_contents(output).unwrap_or_else(|e| {
            eprintln!("failed to set clipboard contents: {}", e);
            std::process::exit(1);
        });
    }
}
