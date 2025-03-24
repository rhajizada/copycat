# copycat

**copycat** is a CLI tool written in **Rust** that reads the contents of a
repository directory (or a single file), formats them into **Markdown**, and
copies the result to your system clipboard.

Each file is embedded in Markdown as a code block, with language-specific syntax
highlighting based on file extension or name. Markdown files themselves are
included as-is. This makes **copycat** especially useful for sharing structured code
snippets with LLMs, documentation tools, or other developers.

## Features

- Recursively collects files from a specified directory.
- Respects `.gitignore` rules.
- Allows additional exclude patterns via `--exclude`.
- Can optionally sort files alphabetically before generating Markdown.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) and Cargo installed.
2. Clone this repository or download the source.
3. Inside the project directory, run:

   ```bash
   cargo install --path .
   ```
