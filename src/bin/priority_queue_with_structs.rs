#![feature(collections)]


use std::fmt::Debug;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// struct representing a book order.  For simplicity it only has an id and the number of books orders
#[derive(Debug, PartialEq, Eq)]
struct BookOrder {
    pub number_of_books: i32,
    pub order_id: i64,
}

impl BookOrder {
    pub fn new(number_of_books: i32, order_id: i64) -> BookOrder {
       BookOrder { number_of_books: number_of_books, order_id: order_id }
   }
}

impl Ord for BookOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number_of_books.cmp(&other.number_of_books)
    }
}

impl PartialOrd for BookOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut heap = BinaryHeap::with_capacity(5);
    heap.push(BookOrder::new(10, 1001));
    heap.push(BookOrder::new(50, 1002));
    heap.push(BookOrder::new(20, 1003));
    heap.push(BookOrder::new(1, 1004));
    heap.push(BookOrder::new(15, 1005));


    let order_with_most_books = heap.peek();
    match order_with_most_books {
         Some(order) => println!("order with most books: {:?}", order), // will print order with id 1002
         None => println!("BinaryHeap is empty")
    }
}
