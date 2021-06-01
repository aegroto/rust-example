use std::fs::File;

use std::env;
use std::io::BufRead;
use std::io::BufReader;

struct Entry {
    angle: u8,
    new_angle: u8,
    hidden_value: u8
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Loading {}...", filename);

    let file = File::open(filename)
                            .expect("Cannot open input file");

    let reader = BufReader::new(file);

    let mut entries: Vec<Entry> = Vec::new();

    let lines = reader.lines().skip(1);

    let value_parser = |fragment: &str| 
        fragment.parse::<u8>().unwrap();

    for line in lines {
        let line = line.unwrap();
        let fragments: Vec<&str> = line.split(',').collect();

        let entry = Entry {
            angle: value_parser(fragments[0]),
            new_angle: value_parser(fragments[1]),
            hidden_value: value_parser(fragments[2])
        };

        entries.push(entry);
    }


}
