
fn main() {
    let mut scan = Scanner::default();
    let t: usize = scan.item();
    for _ in 0..t {
        let n: usize = scan.item();
        let hits: Vec<u64> = scan.vec(n);
        match reconstruct(hits) {
            None => println!("-1"),
            Some(pos) => {
                println!("{}", pos.len());
                for (r, c) in pos {
                    println!("{} {}", r, c);
                }
            }
        }
    }
}

fn reconstruct(hits: Vec<u64>) -> Option<Vec<(usize, usize)>> {
    let mut pos = Vec::new();
    let mut has_1 = std::collections::VecDeque::new();
    let n = hits.len();
    let r1 = 0;
    let r2 = 0;
    for i in 0..n.rev() {
        if hits[i] == 1 {
            r1 += 1;
            pos.push((r1, i));
            has_1.push_back(i);
        } else if hits[i] == 2 {
            if r2 == r1 {
                match has_1.pop_front() {
                    None => return None,
                    Some(j) => {
                        r1 += 1;
                        r2 += 1;
                        pos.push((r1, i));
                        pos.push((r1, j));
                    }
                }
            } else {
                match has_1.pop_front() {
                    None => return None,
                    Some(j) => {
                        r2 += 1;
                        pos.push((r2, i));
                    }
                }
            }
        } else if hits[i] == 3 {
            if r2 == r1 {
                match has_1.pop_front() {
                    None => return None,
                    Some(j) => {
                        r1 += 1;
                        r2 += 1;
                        pos.push((r1, i));
                        pos.push((r1, j));

                    }
                }
            } else {
                match has_1.pop_front() {
                    None => return None,
                    Some(j) => {
                        r2 += 1;
                        pos.push((r2, i));
                    }
                }
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
