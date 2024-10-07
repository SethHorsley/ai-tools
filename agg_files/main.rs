use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [-r] <file_pattern1> [<file_pattern2> ...]", args[0]);
        println!("  -r: Search recursively");
        println!("Example: {} -r '*.{{ts,tsx}}'", args[0]);
        return;
    }

    let mut recursive = false;
    let mut patterns = Vec::new();

    for arg in &args[1..] {
        if arg == "-r" {
            recursive = true;
        } else {
            patterns.push(arg.clone());
        }
    }

    if patterns.is_empty() {
        println!("Please provide at least one file pattern.");
        return;
    }

    let ignored_dirs = get_ignored_dirs();

    for pattern in patterns {
        process_pattern(&pattern, recursive, &ignored_dirs);
    }
}

fn get_ignored_dirs() -> Vec<String> {
    let mut ignored_dirs = Vec::new();
    if let Ok(content) = fs::read_to_string(".gitignore") {
        for line in content.lines() {
            if !line.starts_with('#') && line.ends_with('/') {
                ignored_dirs.push(line.trim_end_matches('/').to_string());
            }
        }
    }
    ignored_dirs
}

fn process_pattern(pattern: &str, recursive: bool, ignored_dirs: &[String]) {
    let regex = glob_to_regex(pattern);
    let walker = if recursive {
        WalkDir::new(".")
    } else {
        WalkDir::new(".").max_depth(1)
    };

    for entry in walker.into_iter().filter_entry(|e| !is_ignored(e, ignored_dirs)) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && regex.is_match(path.to_str().unwrap_or("")) {
                process_file(path);
            }
        }
    }
}

fn is_ignored(entry: &walkdir::DirEntry, ignored_dirs: &[String]) -> bool {
    ignored_dirs.iter().any(|dir| entry.path().starts_with(dir))
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

