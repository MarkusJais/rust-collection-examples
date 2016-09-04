use std::collections::HashMap;


fn main() {
    let a = [0, 1, 2, 11, 444];

    let nums = a.iter().filter(|x| **x > 10).collect::<Vec<&i32>>();

    println!("nums:{:?}", nums);
}