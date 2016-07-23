
use std::fmt::Debug;
use std::collections::BinaryHeap;


// print heap in arbitrary order
fn print_heap<T>(heap: &BinaryHeap<T>) where T: Ord + Debug {
    for x in heap.iter() {
        println!("{:?}", x);
    }
}

fn main() {
    let mut heap = BinaryHeap::with_capacity(5);
    heap.push(4);
    heap.push(14);
    heap.push(24);
    heap.push(8);
    heap.push(18);

    // print it (in arbitrary order)
    print_heap(&heap);

    // print largest number
    let largest_number = heap.pop();
    match largest_number {
        Some(number) => println!("largest_number is: {}", number), // prints 24
        None => println!("BinaryHeap is empty")
    }

    let largest_peek = heap.peek();
    match largest_peek {
        Some(number) => println!("now the largest_number peeked is: {}", number), // now will print 18
        None => println!("BinaryHeap is empty")
    }

    let largest_peek_again = heap.peek();
    match largest_peek_again {
        Some(number) => println!("largest number after 2nd peek still the same: {}", number), // now will print 18
        None => println!("BinaryHeap is empty")
    }
}
