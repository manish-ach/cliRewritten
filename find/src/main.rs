use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn find_files(dir: &Path, query: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    find_files(&path, query);
                }

                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.contains(query) {
                        println!("{}", path.display());
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <search_string> <directory>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    let dir: PathBuf = if args.len() == 3 {
        Path::new(&args[2]).to_path_buf()
    } else {
        env::current_dir().unwrap_or_else(|_| {
            eprintln!("Error: Unable to get the current directory.");
            std::process::exit(1);
        })
    };

    if dir.is_dir() {
        find_files(&dir, query);
    } else {
        eprintln!("Error: {} is not a directory.", dir.display());
        std::process::exit(1);
    }
}
