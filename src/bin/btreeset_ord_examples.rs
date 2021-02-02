use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Person<'a> {
    id: i64,
    firstname: &'a str,
    lastname: &'a str
}

impl<'a> PartialOrd for Person<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.id.partial_cmp(&other.id) }
}

impl<'a> Ord for Person<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.id.cmp(&other.id) }
}


fn main() {
    let mut people = BTreeSet::new();
    let einstein2 = Person {id: 1, firstname: "Albert", lastname: "Einstein"};
    let darwin2 = Person {id: 2, firstname: "Charles", lastname: "Darwin"};
    let bohr2 = Person {id: 1, firstname: "Niels", lastname: "Bohr"};

    people.insert(einstein2);
    people.insert(darwin2);
    people.insert(bohr2);

    println!("people:{:?}", people);
}

