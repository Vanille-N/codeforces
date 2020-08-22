
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let a: Vec<u64> = (0..3).map(|_| scan.next()).collect();
        let b: Vec<u64> = (0..3).map(|_| scan.next()).collect();
        println!("{}", max_score(a, b));
    }
}

fn create(ai: usize, bi: usize, a: &mut Vec<u64>, b: &mut Vec<u64>) -> i64 {
    let cnt = a[ai].min(b[bi]);
    a[ai] -= cnt;
    b[bi] -= cnt;
    // println!("a: {}, b: {}, cnt: {}, remains: {:?} {:?}", ai, bi, cnt, a, b);
    cnt as i64 * ai as i64 * bi as i64 * (if ai > bi { 1 } else if ai < bi { -1 } else { 0 })
}


fn max_score(mut a: Vec<u64>, mut b: Vec<u64>) -> i64 {
    let c21 = create(2, 1, &mut a, &mut b);
    for &(ai, bi) in &[(0, 2), (2, 2), (1, 0), (1, 1), (0, 0), (0, 1), (2, 0)] {
        let _ = create(ai, bi, &mut a, &mut b);
    }
    let c12 = create(1, 2, &mut a, &mut b);
    c21 + c12
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
