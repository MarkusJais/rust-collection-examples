

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.get(4) {
        Some(x) => println!("found something:{}", x),
        None => println!("bad index")
    }
}
