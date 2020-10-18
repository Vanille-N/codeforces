
fn main() {
    let mut scan = Scanner::default();
    let t: usize = scan.item();
    for _ in 0..t {
        let n: usize = scan.item();
        let belts: Vec<char> = scan.vec(n);
        println!("{}", returnable(belts));
    }
}

fn returnable(belts: Vec<char>) -> usize {
    { // phase 1
        let mut seen = None;
        let mut ok = true;
        for &c in &belts {
            match (c, seen) {
                ('-', None) => (),
                (x, None) => seen = Some(x),
                ('-', Some(_)) => (),
                (x, Some(y)) => if x != y { ok = false; break }
            }
        }
        if ok {
            return belts.len();
        }
    }
    { // phase 2
        let mut cnt = 0;
        let n = belts.len();
        if belts[0] == '-' || belts[n-1] == '-' {
            cnt += 1;
        }
        for i in 0..n-1 {
            if belts[i] == '-' || belts[i+1] == '-' {
                cnt += 1;
            }
        }
        return cnt;
    }
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
