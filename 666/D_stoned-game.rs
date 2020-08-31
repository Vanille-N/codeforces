
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.item();
    for _ in 0..t {
        let n: usize = scan.item();
        let a: Vec<u64> = scan.vec(n);
        if win(a) {
            println!("T");
        } else {
            println!("HL");
        }
    }
}

fn win(a: Vec<u64>) -> bool {
    let mut stacks = std::collections::BinaryHeap::new();
    for x in a {
        stacks.push(x);
    }
    let mut playing = true;
    let mut forbidden;
    if let Some(m) = stacks.pop() {
        forbidden = m - 1;
        playing = !playing;
    } else {
        return !playing;
    }
    while let Some(m) = stacks.pop() {
        if forbidden > 0 {
            stacks.push(forbidden);
        }
        forbidden = m - 1;
        playing = !playing;
    }
    !playing
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
