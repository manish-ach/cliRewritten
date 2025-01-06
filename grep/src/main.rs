use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing the argument: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in content: {}", cmd.query, cmd.filename);

    if let Err(e) = run(cmd) {
        println!("error reading the file: {}", e);
        process::exit(1);
    }
}

fn run(cmd: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(cmd.filename)?;
    println!("contents: {}", content);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Argument length less than 3");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
