
fn is_divisible_by_3(n: i64) -> bool {
    n % 3 == 0
}

fn main() {
    let numbers = 1..;  // infinite range of numbers

    // got sum of all numbers up to 20 but slarger than 10 and divisibly by 3
    let sum = numbers.
                take(20).
                skip_while(|&x| x < 10).
                filter(|&x| is_divisible_by_3(x)).
                fold(0, |sum, n| sum + n);

    println!("sum:{}", sum);
}