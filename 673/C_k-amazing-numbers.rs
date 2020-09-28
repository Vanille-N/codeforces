use std::collections::HashMap;

fn main() {
    let mut scan = Scanner::default();
    let t: usize = scan.item();
    for _ in 0..t {
        let n: usize = scan.item();
        let a: Vec<u64> = scan.vec(n);
        //println!("{:?}", &a);
        let ans = amazing(a);
        for i in 1..=n {
            print!("{} ", ans[i].map(|x| x as i64).unwrap_or(-1));
        }
        println!();
    }
}

fn amazing(a: Vec<u64>) -> Vec<Option<usize>> {
    let mut dist = HashMap::new();
    let mut latest = HashMap::new();
    let n = a.len();
    for (i, x) in a.iter().enumerate() {
        let prev = latest.get(x).map(|p| *p + 1).unwrap_or(0);
        let d = i + 1 - prev;
        latest.insert(x, i);
        dist.insert(*x, *dist.get(x).unwrap_or(&0).max(&d));
    }
    for x in 1..n as u64 {
        if latest.get(&x).is_none() { continue ; }
        let prev = latest.get(&x).unwrap_or(&0);
        let d = n - prev;
        dist.insert(x, *dist.get(&x).unwrap_or(&0).max(&d));
    }
    //println!("dist: {:?}", &dist);
    let mut ans = vec![None; n + 1];
    let mut k = n;
    let mut a = 1;
    while k > 0 {
        while a <= n && dist.get(&(a as u64)).unwrap_or(&(n + 1)) > &k {
            a += 1;
        }
        if a < n {
            ans[k] = Some(a);
        }
        k -= 1;
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
