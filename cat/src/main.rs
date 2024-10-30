use colored::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    if args.len() == 2 {
        for line in reader.lines() {
            println!("{}", line?.blue());
        }
    } else if args.len() == 3 {
        let line_str = &args[2];
        let line_num: u8 = line_str.trim().parse().unwrap();

        let mut count = 0;
        for line in reader.lines() {
            count += 1;
            if count == line_num {
                println!("{}", line?.blue());
                return Ok(());
            }
        }
    }

    Ok(())
}
