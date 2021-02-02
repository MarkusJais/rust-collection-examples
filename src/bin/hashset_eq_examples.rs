use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
struct Person<'a> {
    id: i64,
    firstname: &'a str,
    lastname: &'a str
}

impl<'a> Eq for Person<'a> {}

impl<'a> PartialEq for Person<'a> {
    fn eq(&self, other: &Person) -> bool {
        println!("called");
        self.id == other.id
    }
}

impl<'a> Hash for Person<'a> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.id.hash(h);
    }
}

fn main() {
    let mut people = HashSet::new();
    let einstein = Person {id: 1, firstname: "Albert", lastname: "Einstein"};
    let darwin = Person {id: 2, firstname: "Charles", lastname: "Darwin"};
    let bohr = Person {id: 1, firstname: "Niels", lastname: "Bohr"};

    people.insert(einstein);
    people.insert(darwin);
    people.insert(bohr);

    println!("people:{:?}", people);
}

