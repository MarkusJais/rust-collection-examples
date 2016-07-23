use std::collections::BinaryHeap;


fn main() {
    let mut heap_1: BinaryHeap<i32> = BinaryHeap::with_capacity(5);
    let mut heap_2: BinaryHeap<i32> = BinaryHeap::with_capacity(5);

    println!("capacity heap_1:{}", heap_1.capacity());
    println!("capacity heap_2:{}", heap_2.capacity());

    heap_1.reserve(6);
    println!("capacity heap_1 after reserve:{}", heap_1.capacity()); // prints 12 with rust 1.3.0-dev

    heap_2.reserve_exact(6);
    println!("capacity heap_2 after reserve_exact:{}", heap_2.capacity()); // prints 6

    heap_2.push(42); // now one element in heap1
    heap_2.shrink_to_fit();
    println!("capacity heap_2 after shrink_to_fit:{}", heap_2.capacity()); // prints 1

}
