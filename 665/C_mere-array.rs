
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let n: u64 = scan.next();
        let a: Vec<u64> = (0..n).map(|_| scan.next()).collect();
        if is_solvable(a) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn is_solvable(mut a: Vec<u64>) -> bool {
    let min = *a.iter().min().unwrap();
    let valid_idx: Vec<usize> = a.iter()
        .enumerate()
        .filter(|(_, n)| *n % min == 0)
        .map(|(i, _)| i)
        .collect();
    let items = {
        let mut items: Vec<u64> = valid_idx.iter().map(|&i| a[i]).collect();
        items.sort();
        items
    };
    // println!("items: {:?}", &items);
    for (idx, &cnt) in valid_idx.iter().enumerate() {
        a[cnt] = items[idx];
    }
    // println!("arranged: {:?}", &a);
    is_nondecreasing(a)
}

fn is_nondecreasing(a: Vec<u64>) -> bool {
    for i in 1..a.len() {
        if a[i] < a[i-1] {
            return false;
        }
    }
    true
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
