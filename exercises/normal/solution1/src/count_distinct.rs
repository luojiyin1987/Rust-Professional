use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let unique_items: HashSet<&str> = input_str.split(',').collect();
    unique_items.len()
}
