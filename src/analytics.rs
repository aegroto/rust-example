use crate::types::{Alterable, Entry};

use rayon::prelude::*;

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

pub fn calculate_alterations_using_trait(alterables: &Vec<Box<dyn Alterable>>) -> usize {
    alterables 
        .into_iter()
        .filter(|entry| entry.is_altered())
        .count()
}

pub fn parallel_calculate_hidden_values_frequencies(entries: &Vec<Entry>) -> (usize, usize) {
    (
        parallel_calculate_hidden_value_frequency(entries, 0),
        parallel_calculate_hidden_value_frequency(entries, 1),
    )
}

pub fn parallel_calculate_hidden_value_frequency(entries: &Vec<Entry>, value: u8) -> usize {
    entries
        .par_iter()
        .filter(|entry| entry.hidden_value == value)
        .count()
}

pub fn parallel_calculate_alterations(entries: &Vec<Entry>) -> usize {
    entries
        .par_iter()
        .filter(|entry| entry.angle != entry.new_angle)
        .count()
}


