use crate::types::Entry;

pub fn calculate_hidden_values_frequencies(entries: &Vec<Entry>) -> (usize, usize) {
    (
        calculate_hidden_value_frequency(entries, 0),
        calculate_hidden_value_frequency(entries, 1),
    )
}

pub fn calculate_hidden_value_frequency(entries: &Vec<Entry>, value: u8) -> usize {
    entries
        .into_iter()
        .filter(|entry| entry.hidden_value == value)
        .count()
}

pub fn calculate_alterations(entries: &Vec<Entry>) -> usize {
    entries
        .into_iter()
        .filter(|entry| entry.angle != entry.new_angle)
        .count()
}
