use std::fs::File;

use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;

use crate::types::Entry;

pub fn load_entries_from_file(filename: &str) -> Result<Vec<Entry>, ParseIntError> {
    let file = File::open(filename).expect("Cannot open input file");

    let reader = BufReader::new(file);

    let mut entries: Vec<Entry> = Vec::new();

    let lines = reader.lines().skip(1);

    let value_parser = |fragment: &str| fragment.parse::<u8>();

    for line in lines {
        let line = line.unwrap();
        let fragments: Vec<&str> = line.split(',').collect();

        let entry = Entry {
            angle: value_parser(fragments[0])?,
            new_angle: value_parser(fragments[1])?,
            hidden_value: value_parser(fragments[2])?,
        };

        entries.push(entry);
    }

    Ok(entries)
}
