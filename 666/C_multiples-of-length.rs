
fn main() {
    let mut scan = Scanner::default();
    let n: usize = scan.item();
    let a: Vec<i64> = scan.vec(n);
    let mut cpy = a.clone();
    if n == 1 {
        println!("1 1");
        println!("{}", -a[0]);
        println!("1 1");
        println!("0");
        println!("1 1");
        println!("0");
    } else {
        apply(&a, &mut cpy, 0, 0, |_, k| -k);
        apply(&a, &mut cpy, 1, n-1, |_, k| k * (n as i64 - 1));
        apply(&a, &mut cpy, 0, n-1, |i, k| if i == 0 { 0 } else { -k * n as i64 });
        for i in cpy {
            assert_eq!(i, 0);
        }
    }
}

fn apply<F: Fn(usize, i64) -> i64>(a: &Vec<i64>, cpy: &mut Vec<i64>, l: usize, r: usize, f: F) {
    println!("{} {}", l + 1, r + 1);
    for i in l..=r {
        let modify = f(i, a[i]);
        print!("{} ", modify);
        cpy[i] += modify;
    }
    println!();
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
