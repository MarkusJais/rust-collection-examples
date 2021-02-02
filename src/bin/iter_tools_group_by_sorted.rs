extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

fn count_distinct_elements(words: &Vec<&str>) -> HashMap<String, usize> {
    let mut data_grouped: HashMap<String, usize> = HashMap::new();
    for (key, group) in &words.
        into_iter().
        sorted_by(|a, b| Ord::cmp(a, b)).
        into_iter().group_by(|elt| *elt) {
        data_grouped.insert(key.to_string(), group.collect_vec().len());
    }
    return data_grouped;
}


fn main() {
    let words: Vec<&str> = vec!["Rust", "Rust", "Java", "Rust", "Scala", "Java", "Rust", "C++"];
    let count_results = count_distinct_elements(&words);
    for (elem, count) in &count_results {
        println!("{}: \"{}\"", elem, count);
    }
}

