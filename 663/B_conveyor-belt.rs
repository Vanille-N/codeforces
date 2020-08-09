fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let mut cnt = 0;
        let nrows: u64 = scan.next();
        let _ncols: u64 = scan.next();
        for _ in 1..nrows {
            let row: String = scan.next();
            if row.chars().last().unwrap() == 'R' {
                cnt += 1;
            }
        }
        let row: String = scan.next();
        for c in row.chars() {
            if c == 'D' {
                cnt += 1;
            }
        }
        println!("{}", cnt);
    }
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
