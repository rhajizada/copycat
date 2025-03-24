# copycat

**copycat** is a CLI tool written in **Rust** that copies the contents of a
repository directory (or single file) to the system clipboard. This is useful for quickly providing code context to LLMs or sharing snippets.

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
