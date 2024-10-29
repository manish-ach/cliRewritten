use colored::*;
use std::fs;

fn main() {
    let entries = fs::read_dir(".").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        // Get the file name
        let file_name = path.file_name().unwrap().to_string_lossy();

        // Get metadata for additional information
        let metadata = fs::metadata(&path).unwrap();
        let file_type = if metadata.is_dir() {
            "Directory"
        } else {
            "File"
        };

        // Output with color and additional info
        if path.is_dir() {
            println!(" {} - {}", file_name.blue(), file_type.green());
        } else {
            println!(" {} - {}", file_name.red(), file_type.green());
        }
    }
}
