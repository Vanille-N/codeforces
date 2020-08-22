
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let n: u64 = scan.next();
        let k: u64 = scan.next();
        println!("{}", mindist(n, k));
    }
}

fn diff(x: u64, y: u64) -> u64 {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn mindist(n: u64, k: u64) -> u64 {
    let type_2 = {
        if n > k {
            let b = (n - k) / 2;
            let n_target = k + 2 * b;
            n_target
        } else {
            let b = 0;
            let n_target = k + 2 * b;
            n_target
        }
    };
    let type_3 = {
        let b = (n + 2 * k) / 2;
        let n_target = 2 * b - k;
        n_target
    };
    // println!("{} {}", type_2, type_3);
    diff(n, type_2).min(diff(n, type_3))
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
