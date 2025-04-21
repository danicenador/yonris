use std::collections::HashMap;

pub fn count_occurrences(values: Vec<i32>) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();
    for value in values {
        *counts.entry(value).or_insert(0) += 1;
    }
    counts
}
