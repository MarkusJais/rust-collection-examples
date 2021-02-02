
#[derive(Debug, Eq, PartialEq)]
struct BookData<'a> {
    title: &'a str,
    author: &'a str,
    category: &'a str,
}

fn main() {
    let mut books: Vec<BookData> = Vec::new();

    books.push(BookData{title: "On the Origin of Species", author: "Charles Darwin", category: "Evolution"});
    books.push(BookData{title: "The Tangled Bank: An Introduction to Evolution", author: "Carl Zimmer", category: "Evolution"});
    books.push(BookData{title: "Albert Einstein: Su Vida, Su Obra Y Su Mundo", author: "José Manuel Sánchez Ron ", category: "Physics"});

    let evolution_books: Vec<BookData> = books.into_iter().
        filter(| book | book.category == "Evolution").
        collect();

    println!("evolution books: {:?}", evolution_books);

}