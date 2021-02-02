
#[derive(Debug, Eq, PartialEq)]
struct BookData<'a> {
    title: &'a str,
    price: i32,
}


fn main() {

    let mut books: Vec<BookData> = Vec::new();

    books.push(BookData{title: "Book 1", price: 10});
    books.push(BookData{title: "Book 2", price: 20});
    books.push(BookData{title: "Book 3", price: 30});

    let total_price: i32 = books.iter()
        .map(|book| book.price * 2)
        .sum();

    println!("price with taxes: {:?}", total_price);

}


