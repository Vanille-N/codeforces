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
    let nb_tests = scan.next::<usize>();
    for _ in 0..nb_tests {
        let n = scan.next::<usize>();
        let vals: Vec<usize> = (0..n).map(|_| scan.next()).collect();
        if is_solvable(vals) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn close_enough(a: usize, b: usize) -> bool {
    (a as isize - b as isize).abs() <= 1
}

fn is_solvable(mut vals: Vec<usize>) -> bool {
    vals.sort();
    let mut ok = true;
    for i in 0..vals.len()-1 {
        if !close_enough(vals[i], vals[i+1]) {
            return false;
        }
    }
    true
}
