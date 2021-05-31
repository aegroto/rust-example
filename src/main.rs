use std::fs::File;

use std::env;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Loading {}...", filename);

    let file = File::open(filename)
                            .expect("Cannot open input file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
