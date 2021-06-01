use std::env;

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

    println!("Calculating some analytics...");

    let hidden_values_frequencies = analytics::calculate_hidden_values_frequencies(&entries); 
    println!("Hidden values frequencies: {:?}", hidden_values_frequencies);

    let alterations = analytics::calculate_alterations(&entries);
    println!("Alterations: {}/{}", alterations, entries.len());
}
