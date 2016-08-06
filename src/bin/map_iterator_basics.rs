use std::collections::HashMap;
//use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct BookData<'a> {
    author: &'a str,
    category: &'a str,
    price_in_euro: i32
}

fn main() {
    let mut books: HashMap<&str, BookData> = HashMap::new();
    books.insert("On the Origin of Species", BookData{author: "Charles Darwin", category: "Evolution", price_in_euro: 10});
    books.insert("The Tangled Bank: An Introduction to Evolution", BookData{author: "Carl Zimmer", category: "Evolution", price_in_euro: 40});
    books.insert("Albert Einstein: Su Vida, Su Obra Y Su Mundo", BookData{author: "José Manuel Sánchez Ron ", category: "Physics", price_in_euro: 35});

    let evolution_books: HashMap<&str, BookData> = books.into_iter().
        filter(|&(_, ref v)| v.category == "Evolution").
        collect();

    for b in evolution_books {
        println!("{:?}", b);
    }
}