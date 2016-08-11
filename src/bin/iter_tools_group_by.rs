#[macro_use] extern crate itertools;

use itertools::Itertools;

fn main() {

    let languages = vec!["Rust", "Rust", "Scala", "Python", "Rust", "C++", "Java", "Rust", "Python"];

    let res = languages.into_iter().group_by(|elem| *elem);

    for (k, v) in res {
        println!("{:?}:{:?}", k, v.len());
    }
}