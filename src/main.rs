use std::env;

use crate::analytics::calculate_hidden_values_frequencies;

mod load;
mod types;
mod analytics;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Loading entries from {}...", filename);

    let entries = match load::load_entries_from_file(filename) {
        Ok(entries) => entries,
        Err(_) => panic!("Input parsing error")
    };

    let hidden_values_frequencies = calculate_hidden_values_frequencies(&entries); 

    println!("Hidden values frequencies: {:?}", hidden_values_frequencies);
}
