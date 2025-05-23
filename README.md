# copycat

![ci](https://github.com/rhajizada/copycat/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

CLI tool to copy your project source code as Markdown to clipboard for
context-aware responses from LLMs.

## Features

- Recursive file collection from a specified directory or single file
- Respects .gitignore
- Custom exclude patterns via --exclude <pattern>
- Optional sorting (--sort)
- Output modes:
  - Markdown: embeds each file as a fenced code block with syntax highlighting
  - Tree: renders an ASCII-style directory tree (--tree)
- Choose between copying to clipboard (default) or printing to stdout (--print)

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) and Cargo installed.
2. Clone this repository or download the source.
3. Inside the project directory, run:

   ```bash
   cargo install --path .
   ```
