
use std::fmt::Debug;

fn print_vec<T>(v: Vec<T>) where T: Debug {
    for n in v {
        print!("{:?}, ", n);
    }
    println!("");
}

fn main() {
    let numbers = vec![1, 3, 7, 100, 42000];
    let numbers_doubled: Vec<i32> = numbers.iter().map(|x| x  * 2).collect();
    print_vec(numbers_doubled);
    print_vec(numbers);
}
