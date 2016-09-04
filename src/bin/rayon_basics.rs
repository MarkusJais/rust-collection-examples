extern crate rayon;
use rayon::prelude::*;


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

    let total_price: i32 = books.par_iter()
        .map(|book| book.price * 2)
        .sum();

    println!("rayon price with taxes: {}", total_price);

    let numbers = vec![10, 20, 30, 40, 50];
    let sum: i32 = numbers.par_iter()
        .map(|n| n * 2)
        .sum();

    println!("rayon sum: {}", sum);

}


