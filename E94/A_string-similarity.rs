
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let n: u64 = scan.next();
        let bits: Vec<bool> = scan.next::<String>().chars().map(|c| c == '1').collect();
        println!("{}", cantor(n, bits).iter().map(|b| if *b { '1' } else { '0' }).collect::<String>());
    }
}

fn cantor(n: u64, v: Vec<bool>) -> Vec<bool> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(v[i as usize * 2]);
    }
    res
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
