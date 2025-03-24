mod files;
mod formatter;
mod language;

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
    #[arg(long = "sort", action = ArgAction::SetTrue)]
    sort: bool,
}

fn main() {
    let args = Args::parse();

    if !args.path.exists() {
        panic!("provided path {} does not exist", args.path.display());
    }

    let files = files::collect_files(args.path.clone(), &args.excludes, args.sort)
        .expect("failed to collect files");

    if files.is_empty() {
        eprintln!("no matching files found, nothing to copy");
        std::process::exit(1);
    }

    let markdown = formatter::generate_markdown(&args.path, &files)
        .expect("failed to format files as Markdown");

    let mut ctx = ClipboardContext::new().expect("failed to create clipboard context");
    ctx.set_contents(markdown)
        .expect("failed to set clipboard contents");
}
