use std::{env, process};
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the argument: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in content:\n {}", config.query, config.filename);

    if let Err(e) = grep::run(config) {
        eprintln!("error reading the file: {}", e);
        process::exit(1);
    }
}
