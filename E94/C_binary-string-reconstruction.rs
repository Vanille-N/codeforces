
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let s: Vec<bool> = scan.next::<String>().chars().map(|c| c == '1').collect();
        let x: u64 = scan.next();
        if let Some(ans) = reconstruct(s, x as usize) {
            println!("{}", ans.iter().map(|b| if *b { '1' } else { '0' }).collect::<String>());
        } else {
            println!("-1");
        }
    }
}

fn reconstruct(s: Vec<bool>, x: usize) -> Option<Vec<bool>> {
    let n = s.len();
    let mut ans: Vec<bool> = vec![true; n];
    for i in 0..n {
        if !s[i] {
            if i >= x {
                ans[i - x] = false;
            }
            if i + x < n {
                ans[i + x] = false;
            }
        }
    }
    for i in 0..n {
        if s[i] {
            if !((i >= x && ans[i - x]) || (i + x < n && ans[i + x])) {
                return None;
            }
        }
    }
    Some(ans)
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
