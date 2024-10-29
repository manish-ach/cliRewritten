use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    for filename in args {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}
