const MODULO: u64 = 1000000007;

fn main() {
    let mut scan = Scanner::default();
    let n: u64 = scan.next();
    count_cyclic(n);
}

fn count_cyclic(n: u64) {
    let nb_perm = count_permutations(n);
    let nb_acyc = count_acyclic(n);
    println!("{}", (nb_perm as i64 - nb_acyc as i64).rem_euclid(MODULO as i64));
}

fn count_permutations(n: u64) -> u64 {
    let mut fac = 1;
    for i in 1..=n {
        fac = (fac * i) % MODULO;
    }
    fac
}

fn count_acyclic(n: u64) -> u64 {
    let mut pred = 2;
    for _ in 3..=n {
        pred = (2 * pred) % MODULO;
    }
    pred
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
