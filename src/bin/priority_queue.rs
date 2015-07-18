#![feature(collections)]

use std::collections::BinaryHeap;
fn main() {
    let mut heap = BinaryHeap::with_capacity(10);
    heap.push(4);
    heap.push(14);
    heap.push(24);
    heap.push(8);
    heap.push(18);

    // print it (in arbitrary order)
    for x in heap.iter() {
        println!("{}", x);
    }

    // print largest number
    let mut largest_number = heap.pop();
    match largest_number {
        Some(number) => println!("largest_number is:{}", number), // prints 24
        None => println!("BinaryHeap is empty")
    }

    largest_number = heap.pop();
    match largest_number {
        Some(number) => println!("now the largest_number is:{}", number), // now will print 18
        None => println!("BinaryHeap is empty")
    }

    let vec_unsorted = heap.clone().into_vec();

    println!("unsorted vec:{:?}", vec_unsorted);

    let vec_sorted = heap.into_sorted_vec();

    println!("sorted vec:{:?}", vec_sorted);

}
