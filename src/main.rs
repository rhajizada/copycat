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
    /// The path to a repository directory or a single file.
    /// If a directory, copycat recursively collects files.
    path: PathBuf,

    /// One or more glob patterns for excluding files (e.g. "*.DS_Store", "**/*.md").
    /// Can be repeated multiple times.
    #[arg(long = "exclude", short = 'e', action = ArgAction::Append)]
    excludes: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if !args.path.exists() {
        panic!("provided path \"{}\" does not exist", args.path.display());
    }

    let files =
        files::collect_files(args.path.clone(), &args.excludes).expect("Failed to collect files");

    if files.is_empty() {
        eprintln!("no matching files found, nothing to copy");
        std::process::exit(1);
    }

    let markdown = formatter::generate_markdown(&args.path, &files)
        .expect("Failed to format files as Markdown");

    let mut ctx = ClipboardContext::new().expect("Failed to create clipboard context");
    ctx.set_contents(markdown)
        .expect("Failed to set clipboard contents");
}
