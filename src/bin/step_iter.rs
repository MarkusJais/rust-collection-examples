#[derive(Debug)]
struct Step {
    num: i64,
    next: i64,
    step: i64
}

impl Step {
    fn new(start: i64, step: i64) -> Step {
        Step{num: start, next: step, step: step}
    }
}

impl Iterator for Step {
    type Item = i64;

    fn next (&mut self) -> Option<i64> {
        self.num = self.next;
        let new_next = self.num + self.step;
        self.next = new_next;
        Some(self.num)
    }
}

fn main() {
    let e = Step::new(0, 5);
    for i in e.take(5) {
        println!("{:?}", i);
    }
}