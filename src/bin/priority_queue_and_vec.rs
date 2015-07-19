#![feature(collections)]

use std::collections::BinaryHeap;


fn main() {
    let mut heap = BinaryHeap::with_capacity(5);
    heap.push(4);
    heap.push(14);
    heap.push(24);
    heap.push(8);
    heap.push(18);

    let vec_unsorted = heap.clone().into_vec();

    println!("unsorted vec:{:?}", vec_unsorted);

    let vec_sorted = heap.into_sorted_vec();

    println!("sorted vec:{:?}", vec_sorted);
}
