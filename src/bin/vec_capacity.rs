// this code shows the effects of different methods on the capcacity of Vec
// the different examles are separated by "====================================="
// if you find a bug please open a pull request


fn main() {

    println!("\n1) ==========================================\n");

    let mut vec: Vec<i32> = Vec::new();  // empty vector has capacity of 0

    println!("capacity vec:{}", vec.capacity());

    vec.reserve(8); // now capacity of 16 because vector empty and 8 will be multiplied with 2
    println!("capacity vec after reserve(8):{}", vec.capacity());

    vec.clear();  // clear does not affect capacity
    println!("capacity vec after clear:{}", vec.capacity());

    vec.shrink_to_fit();  // because vector has no elements capacity not 0 again
    println!("capacity vec after clear and shrink_to_fit:{}", vec.capacity());

    println!("vec:{:?}", vec);

    println!("\n2) ==========================================\n");



    let mut vec: Vec<i32> = Vec::new();  // empty
    vec.push(100);  // empty vec with capacity 0 will resize to capacity of 4 when one element is added
    println!("capacity vec after pushing 1 element to empty vec:{}", vec.capacity());

    let mut vec: Vec<i32> = vec![1];  // capacity is 4Vec::new();  // empty
    println!("capacity vec after macro initialization with 1 element: {}", vec.capacity());
    vec.push(100); // non empty vectors resize capacity to double their currenty capcacity
    println!("capacity vec after macro initialization with 1 element and pushing 1 element:{}", vec.capacity());

    let mut vec: Vec<i32> = vec![1,2,3,4];  // capacity is 4
    println!("capacity vec after macro initialization with 4 elements: {}", vec.capacity());
    vec.push(100); // small vectors resize to a capacity of at least 4
    println!("capacity vec after macro initialization with 4 element and pushing 1 element:{}", vec.capacity());


    println!("\n3) ==========================================\n");



    let mut vec: Vec<i32> = Vec::new();  // empty
    vec.push(100);  // empty vec with capacity 0 will resize to capacity of 4 when one element is added
    println!("capacity vec after pushing 1 element to empty vec:{}", vec.capacity());
    vec.reserve(8);  // 1 element in the vector + 8 reserved and then multiplied by 2. (1 + 8) * 2 = 19
    println!("capacity vec after pushing 1 element and reserve(8):{}", vec.capacity());

    vec.shrink_to_fit(); // now capacity is as big as the number of elements, in this case 1
    println!("capacity vec after shrink_to_fit:{}", vec.capacity());

    println!("vec:{:?}", vec);  // just to prove that the element is still there after shrinking :-)


    println!("\n4) ==========================================\n");



    let mut vec: Vec<i32> = Vec::with_capacity(5);
    println!("capacity vec:{}", vec.capacity());

    vec.push(1);  // adding one element to capacity with
    println!("capacity after adding 1 element to empty vec with capacity 5:{}", vec.capacity());

    vec.push(2);
    vec.push(3);
    println!("capacity after adding 3 element to empty vec with capacity 5:{}", vec.capacity());

    vec.push(4);
    vec.push(5); // still no resize because number of elements is <= capacity
    println!("capacity after adding 5 element to empty vec with capacity 5:{}", vec.capacity());

    vec.push(6);  // now resizing happens
    println!("capacity after adding 6 element to empty vec with capacity 5:{}", vec.capacity());

    println!("\n5) ==========================================\n");



    let mut vec: Vec<i32> = Vec::with_capacity(5);
    vec.reserve(4); // still capacity of 5
    println!("capacity of empty vec with initial capacity of 5 after reserve(4):{}", vec.capacity());

    vec.reserve(6); // now capacity of 12 (6 * 2)
    println!("capacity of empty vec with initial capacity of 5 after reserve(6):{}", vec.capacity());

    println!("\n6) ==========================================\n");



    let mut vec: Vec<i32> = Vec::with_capacity(5);
    vec.push(1);
    vec.push(2);
    vec.reserve(4); // will result in a capacity of 12. (2 + 4) * 2
    println!("capacity vec with initial capacity of 5 after adding 2 elements and reserve(4):{}", vec.capacity());

    vec.reserve(6); // no changes because capacity is already 12
    println!("capacity vec with initial capacity of 5 after adding 2 and reserve(4) and then reserve(6):{}", vec.capacity());

    println!("\n7) ==========================================\n");



    let mut vec: Vec<i32> = Vec::with_capacity(5);
    vec.push(1);
    vec.push(2);
    vec.reserve_exact(3); // 2 elements added and 3 reserved will keep the initial capacity of 5
    println!("capacity vec with initial capacity of 5 after adding 2 elements and reserve_exact(3):{}", vec.capacity());

    println!("\n8) ==========================================\n");



    let mut vec: Vec<i32> = Vec::with_capacity(5);
    vec.push(1);
    vec.push(2);
    vec.reserve_exact(4); // capacity now 6. 2 elements already inside the vector and reserving exactly 4 more
    println!("capacity vec with initial capacity of 5 after adding 2 elements and reserve_exact(4):{}", vec.capacity());



    println!("\n Done!\n");
}
