
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let n: u64 = scan.next();
        let k: u64 = scan.next();
        let tab: Vec<i64> = (0..n).map(|_| scan.next()).collect();
        apply(tab, k).iter().for_each(|a| print!("{} ", a));
        println!();
    }
}

fn apply(mut tab: Vec<i64>, k: u64) -> Vec<i64> {
    if k == 0 {
        tab
    } else if k == 1 {
        flip(&mut tab);
        tab
    } else {
        flip(&mut tab);
        flip(&mut tab);
        if k % 2 == 1 {
            flip(&mut tab);
        }
        tab
    }
}

fn flip(tab: &mut Vec<i64>) {
    let sup = *tab.iter().max().unwrap();
    for i in 0..tab.len() {
        tab[i] = sup - tab[i];
    }
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
