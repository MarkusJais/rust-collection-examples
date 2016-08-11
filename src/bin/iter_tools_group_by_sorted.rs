extern crate itertools;

use itertools::Itertools;


fn main() {
    let words = vec!["Rust", "Rust", "Java", "Rust", "Scala", "Java", "Rust", "C++"];

    for (key, group) in words.
        into_iter().
        sorted_by(|a, b| Ord::cmp(a, b)).
        into_iter().group_by(|elt| *elt) {
        println!("{}: {:?}", key, group);
    }
}
