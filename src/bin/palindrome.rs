
fn is_numerical_palindrome(num: i64) -> bool {
    let str = num.to_string();
    let reverse_str: String = str.chars().rev().collect();
    str == reverse_str
}

fn is_prime(num: i64) -> bool {
    if num == 1 {
        return true;
    }
    let sqrt: i64 = (num as f64).sqrt() as i64;
    for x in 2..sqrt + 1 {
        if num % x == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num_range = 1..;
    let numbers = num_range.into_iter().
                    filter(|&num| is_numerical_palindrome(num)).
                    filter(|&num| is_prime(num)).
                    take(10).
                    collect::<Vec<i64>>();


    println!("{:?}", numbers);


}