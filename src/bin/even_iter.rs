#[derive(Debug)]
struct Even {
    num: i64,
    next: i64
}

impl Iterator for Even {
    type Item = i64;

    fn next (&mut self) -> Option<i64> {
        self.num = self.next;
        let new_next = self.num + 2;
        self.next = new_next;
        Some(self.num)
    }
}

fn main() {
    let e = Even{num: 0, next: 2};
    for i in e.take(5) {
        println!("{:?}", i);
    }


}