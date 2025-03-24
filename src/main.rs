mod files;
mod formatter;
mod language;

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "copycat",
    version,
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    if !args.path.exists() {
        panic!("provided path \"{}\" does not exist", args.path.display());
    }
    let files = files::collect_files(args.path.clone()).unwrap();
    let markdown = formatter::generate_markdown(&args.path, &files).unwrap();
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(markdown.to_owned()).unwrap();
}
