use std::collections::HashMap;

fn main() {
    let mut scan = Scanner::default();
    let t: usize = scan.item();
    for _ in 0..t {
        let (len, sum): (usize, u64) = scan.item();
        let items: Vec<u64> = scan.vec(len);
        let answer: Vec<bool> = minimize(items, sum);
        answer.into_iter().for_each(|b| print!("{} ", if b { 1 } else { 0 }));
        println!();
    }
}

fn minimize(items: Vec<u64>, sum: u64) -> Vec<bool> {
    let mut ans = Vec::new();
    let mut selected = HashMap::new();
    let mut rejected = HashMap::new();
    for x in items {
        if x > sum {
            ans.push(true);
            continue;
        }
        let corresp = sum - x;
        let sel_penalty = selected.get(&corresp).unwrap_or(&0);
        let rej_penalty = rejected.get(&corresp).unwrap_or(&0);
        if sel_penalty > rej_penalty {
            ans.push(false);
            rejected.insert(x, rejected.get(&x).unwrap_or(&0) + 1);
        } else {
            ans.push(true);
            selected.insert(x, selected.get(&x).unwrap_or(&0) + 1);
        }
    }
    ans
}

// TEMPLATE

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn atomic_scan<T: AtomicScannable>(&mut self) -> T {
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
trait AtomicScannable: std::str::FromStr {}
impl AtomicScannable for u64 {}
impl AtomicScannable for i64 {}
impl AtomicScannable for f64 {}
impl AtomicScannable for usize {}
impl AtomicScannable for isize {}
impl AtomicScannable for String {}
trait Scan<T> {
    fn item(&mut self) -> T;
}
impl<T: AtomicScannable> Scan<T> for Scanner {
    fn item(&mut self) -> T {
        self.atomic_scan()
    }
}
impl<T, U> Scan<(T, U)> for Scanner where Scanner: Scan<T> + Scan<U> {
    fn item(&mut self) -> (T, U) {
        (self.item(), self.item())
    }
}
impl<T, U, V> Scan<(T, U, V)> for Scanner where Scanner: Scan<T> + Scan<U> + Scan<V> {
    fn item(&mut self) -> (T, U, V) {
        (self.item(), self.item(), self.item())
    }
}
trait ScanVec<T> {
    fn vec(&mut self, n: usize) -> Vec<T>;
}
impl<T> ScanVec<T> for Scanner where Scanner: Scan<T> {
    fn vec(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.item()).collect()
    }
}
impl ScanVec<bool> for Scanner {
    fn vec(&mut self, n: usize) -> Vec<bool> {
        let s: String = self.item();
        let v: Vec<bool> = s.chars().map(|c| c == '1').collect();
        if v.len() != n {
            panic!("Wrong length for Vec<bool>: got {}, expected {}", v.len(), n);
        }
        v
    }
}
impl ScanVec<char> for Scanner {
    fn vec(&mut self, n: usize) -> Vec<char> {
        let s: String = self.item();
        let v: Vec<char> = s.chars().collect();
        if v.len() != n {
            panic!("Wrong length for Vec<char>: got {}, expected {}", v.len(), n);
        }
        v
    }
}
