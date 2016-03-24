struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 1, next: 1}
}

fn main() {
    let result: u32 = fibonacci().take_while(|&x| x <= 4000000).filter(|x| x % 2 == 0).fold(0, |sum, x| sum + x);
    println!("{}", result.to_string())
}
