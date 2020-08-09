use std::io::stdin;

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
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let nb_tests = scan.next::<u64>();
    for _ in 0..nb_tests {
        let n = scan.next::<u64>();
        let a = (0..n).map(|_| scan.next::<u64>()).collect::<Vec<_>>();
        let b = (0..n).map(|_| scan.next::<u64>()).collect::<Vec<_>>();
        let gifts = a.into_iter().zip(b.into_iter()).collect::<Vec<_>>();
        println!("{}", min_moves(gifts));
    }
}

fn min_moves(gifts: Vec<(u64, u64)>) -> u64 {
    let (a, b) = {
        let (mut a, mut b) = gifts[0];
        for &(ai, bi) in gifts.iter() {
            a = a.min(ai);
            b = b.min(bi);
        }
        (a, b)
    };
    let mut cnt = 0;
    for (ai, bi) in gifts {
        let a_dist = ai - a;
        let b_dist = bi - b;
        cnt += a_dist.max(b_dist);
    }
    cnt
}
