

fn main() {
    let xs = [1, 2, 3, 4, 5];

    let mut iter = xs.iter().peekable();

    println!("{:?}", iter.peek());
    println!("{:?}", iter.peek());
    println!("{:?}", iter.peek());

    //println!("{:?}", iter.take(2));

    println!("{:?}", iter.next());

    let a = [1, 2, 3];

    let mut iter2 = a.iter().peekable();

    println!("i2: {:?}", iter2.next());
    println!("i2: {:?}", iter2.next());

}