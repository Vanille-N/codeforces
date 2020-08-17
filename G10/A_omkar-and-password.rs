
fn main() {
    let mut scan = Scanner::default();
    let t = scan.next::<u64>();
    for _ in 0..t {
        let n = scan.next::<u64>();
        let pass: Vec<u64> = (0..n).map(|_| scan.next()).collect();
        println!("{}", reduce(pass));
    }
}

fn reduce(pass: Vec<u64>) -> u64 {
    let fst = pass[0];
    for x in pass.iter() {
        if *x != fst {
            return 1;
        }
    }
    pass.len() as u64
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
