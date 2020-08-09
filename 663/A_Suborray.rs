
fn main() {
    let mut scan = Scanner::default();
    let t = scan.next::<u64>();
    for _ in 0..t {
        let n = scan.next::<u64>();
        solve(n);
    }
}

fn solve(n: u64) {
    (1..=n).for_each(|i| print!("{} ", i));
    println!();
}

// TEMPLATE

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
