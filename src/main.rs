mod files;
mod formatter;
mod language;

use anyhow::Result;
use clap::{ArgAction, Parser};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::path::PathBuf;

/// Command-line arguments for the `copycat` application.
#[derive(Parser, Debug)]
#[command(
    name = "copycat",
    version,
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    /// Path to repository directory or a single file.
    /// If directory, copycat recursively collects files.
    path: PathBuf,

    /// One or more glob patterns for excluding files (e.g. ".gitignore", "**/*.md").
    /// Can be repeated multiple times.
    #[arg(long = "exclude", short = 'e', action = ArgAction::Append)]
    excludes: Vec<String>,

    /// Sort files alphabetically.
    #[arg(long = "sort", short = 's', action = ArgAction::SetTrue)]
    sort: bool,
}

fn run(args: Args) -> Result<()> {
    if !args.path.exists() {
        anyhow::bail!("provided path {} does not exist", args.path.display());
    }

    let files = files::collect_files(args.path.clone(), &args.excludes, args.sort)
        .map_err(|e| anyhow::anyhow!("failed to collect files: {}", e))?;

    if files.is_empty() {
        anyhow::bail!("no matching files found, nothing to copy");
    }

    let markdown = formatter::generate_markdown(&args.path, &files)
        .map_err(|e| anyhow::anyhow!("failed to read files: {}", e))?;

    let mut ctx = ClipboardContext::new()
        .map_err(|e| anyhow::anyhow!("failed to create clipboard context: {}", e))?;

    ctx.set_contents(markdown)
        .map_err(|e| anyhow::anyhow!("failed to set clipboard contents: {}", e))?;

    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
