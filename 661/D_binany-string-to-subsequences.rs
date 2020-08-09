use std::io::stdin;

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
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let t = scan.next::<u64>();
    for _ in 0..t {
        let _ = scan.next::<u64>();
        let bin = scan.next::<String>().chars().map(|c| c == '1').collect::<Vec<_>>();
        let (n, distrib) = subdivide(bin);
        println!("{}", n);
        for k in distrib {
            print!("{} ", k);
        }
        println!();
    }
}

fn subdivide(bin: Vec<bool>) -> (u64, Vec<u64>) {
    let mut cnt = 0;
    let mut idx = Vec::new();
    let mut ones = Vec::new();
    let mut zeros = Vec::new();
    for b in bin {
        if b {
            if let Some(k) = zeros.pop() {
                idx.push(k);
                ones.push(k);
            } else {
                cnt += 1;
                idx.push(cnt);
                ones.push(cnt);
            }
        } else {
            if let Some(k) = ones.pop() {
                idx.push(k);
                zeros.push(k);
            } else {
                cnt += 1;
                idx.push(cnt);
                zeros.push(cnt);
            }
        }
    }
    (cnt, idx)
}
