use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;
use ignore::gitignore::{GitignoreBuilder, Gitignore};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [-r] [-i] <file_pattern1> [<file_pattern2> ...]", args[0]);
        println!("  -r: Search recursively");
        println!("  -i: Ignore .gitignore (include all files)");
        println!("Example: {} -r '*.{{ts,tsx}}'", args[0]);
        return;
    }

    let mut recursive = false;
    let mut ignore_gitignore = false;
    let mut patterns = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "-r" => recursive = true,
            "-i" => ignore_gitignore = true,
            _ => patterns.push(arg.clone()),
        }
    }

    if patterns.is_empty() {
        println!("Please provide at least one file pattern.");
        return;
    }

    let gitignore = if !ignore_gitignore {
        build_gitignore()
    } else {
        None
    };

    for pattern in patterns {
        if pattern == "*" || pattern == "./*" {
            process_all_files(recursive, &gitignore);
        } else {
            process_pattern(&pattern, recursive, &gitignore);
        }
    }
}

fn build_gitignore() -> Option<Gitignore> {
    let mut builder = GitignoreBuilder::new(".");
    if builder.add(".gitignore").is_none() {
        builder.build().ok()
    } else {
        None
    }
}

fn process_all_files(recursive: bool, gitignore: &Option<Gitignore>) {
    let walker = if recursive {
        WalkDir::new(".")
    } else {
        WalkDir::new(".").max_depth(1)
    };

    for entry in walker.into_iter().filter_entry(|e| !is_ignored(e.path(), gitignore)) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                process_file(path);
            }
        }
    }
}

fn process_pattern(pattern: &str, recursive: bool, gitignore: &Option<Gitignore>) {
    let regex = glob_to_regex(pattern);
    let walker = if recursive {
        WalkDir::new(".")
    } else {
        WalkDir::new(".").max_depth(1)
    };

    for entry in walker.into_iter().filter_entry(|e| !is_ignored(e.path(), gitignore)) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && regex.is_match(path.to_str().unwrap_or("")) {
                process_file(path);
            }
        }
    }
}

fn is_ignored(path: &Path, gitignore: &Option<Gitignore>) -> bool {
    if let Some(gi) = gitignore {
        gi.matched(path, path.is_dir()).is_ignore()
    } else {
        false
    }
}

fn process_file(path: &Path) {
    println!("# File: {}", path.display());
    if let Ok(contents) = fs::read_to_string(path) {
        println!("{}", contents);
        println!("\n================================================================================\n");
    } else {
        println!("Error reading file: {}", path.display());
    }
}

fn glob_to_regex(pattern: &str) -> Regex {
    let regex_str = pattern
        .replace(".", "\\.")
        .replace("*", ".*")
        .replace("{", "(")
        .replace("}", ")")
        .replace(",", "|")
        .replace(" ", "");  // Remove spaces
    Regex::new(&format!(".*{}$", regex_str)).unwrap()
}
