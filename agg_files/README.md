# agg_files

A simple Rust script to aggregate file contents based on glob patterns.

## Usage

```bash
agg_files [-r] <file_pattern1> [<file_pattern2> ...]
```

- `-r`: Search recursively
- `<file_pattern>`: Glob pattern for files to aggregate

## Examples

```bash
agg_files ".rs"
agg_files -r "src/**/.{rs,toml}"
agg_files "lib/rbui/command/*.{rb,js}" -r
```

## Features

- Supports multiple file patterns
- Recursive search option
- Respects `.gitignore` for directory exclusions
- Prints file contents with clear separators

## Installation

Compile the Rust script and ensure it's in your PATH.

## Dependencies

- walkdir
- regex
