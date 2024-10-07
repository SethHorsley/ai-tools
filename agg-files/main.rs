use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;
use std::process::Command;

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

    let is_git_repo = is_git_repository();
    let ignored_dirs = if is_git_repo {
        get_ignored_dirs()
    } else {
        Vec::new()
    };

    for pattern in patterns {
        if pattern == "*" || pattern == "./*" {
            if is_git_repo {
                process_git_files(recursive, &ignored_dirs);
            } else {
                process_all_files(recursive, &ignored_dirs);
            }
        } else {
            process_pattern(&pattern, recursive, &ignored_dirs);
        }
    }
}

fn is_git_repository() -> bool {
    Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
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

fn process_git_files(recursive: bool, ignored_dirs: &[String]) {
    let output = Command::new("git")
        .args(&["ls-files"])
        .output()
        .expect("Failed to execute git command");

    let files = String::from_utf8_lossy(&output.stdout);
    for file in files.lines() {
        let path = Path::new(file);
        if recursive || path.parent().map_or(true, |p| p == Path::new("")) {
            if !is_ignored(path, ignored_dirs) {
                process_file(path);
            }
        }
    }
}

fn process_all_files(recursive: bool, ignored_dirs: &[String]) {
    let walker = if recursive {
        WalkDir::new(".")
    } else {
        WalkDir::new(".").max_depth(1)
    };

    for entry in walker.into_iter().filter_entry(|e| !is_ignored(e.path(), ignored_dirs)) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                process_file(path);
            }
        }
    }
}

fn process_pattern(pattern: &str, recursive: bool, ignored_dirs: &[String]) {
    let regex = glob_to_regex(pattern);
    let walker = if recursive {
        WalkDir::new(".")
    } else {
        WalkDir::new(".").max_depth(1)
    };

    for entry in walker.into_iter().filter_entry(|e| !is_ignored(e.path(), ignored_dirs)) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && regex.is_match(path.to_str().unwrap_or("")) {
                process_file(path);
            }
        }
    }
}

fn is_ignored(path: &Path, ignored_dirs: &[String]) -> bool {
    ignored_dirs.iter().any(|dir| path.starts_with(dir))
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
