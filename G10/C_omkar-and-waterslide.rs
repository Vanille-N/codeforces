
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let n: u64 = scan.next();
        let slide: Vec<u64> = (0..n).map(|_| scan.next()).collect();
        println!("{}", steps(slide));
    }
}

fn steps(slide: Vec<u64>) -> u64 {
    let mut step = 0;
    for i in (1..slide.len()).rev() {
        step += slide[i-1].saturating_sub(slide[i]);
    }
    step
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
