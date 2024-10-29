use colored::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let entries = fs::read_dir(".").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();
        let file_type = if metadata.is_dir() {
            "Directory"
        } else {
            "File"
        };

        // Get file size
        let file_size = metadata.len();

        // Get modified time
        let modified_time = metadata.modified().unwrap();
        let duration = modified_time.duration_since(UNIX_EPOCH).unwrap();
        let seconds = duration.as_secs();

        // Output with color and additional info
        if path.is_dir() {
            println!(
                "{} - {} - {} bytes - {} seconds ago",
                path.display().to_string().blue(),
                file_type,
                file_size,
                seconds
            );
        } else {
            println!(
                "{} - {} - {} bytes",
                path.display().to_string(),
                file_type,
                file_size
            );
        }
    }
}
