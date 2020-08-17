#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orient {
    Lt,
    Rt,
}

fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let _: u64 = scan.next();
        let play: Vec<_> = scan.next::<String>().chars().map(|c| match c {
            'L' => Orient::Lt,
            'R' => Orient::Rt,
            _ => unreachable!(),
        }).collect();
        println!("{}", count(play));
    }
}

fn count(play: Vec<Orient>) -> u64 {
    let mut acc = Vec::new();
    acc.push((play[0], 1));
    for x in play.iter().skip(1) {
        let (top, cnt) = acc.pop().unwrap();
        if top != *x {
            acc.push((top, cnt));
            acc.push((*x, 1));
        } else {
            acc.push((top, cnt+1));
        }
    }
    if acc.len() > 1 {
        let (top, cnt) = acc.pop().unwrap();
        if top == acc[0].0 {
            acc[0] = (top, acc[0].1 + cnt);
        } else {
            acc.push((top, cnt));
        }
        // println!("{:?}", acc);
        acc.iter().map(|(_, cnt)| cnt / 3).sum::<u64>()
    } else {
        // println!("{:?}", acc);
        (acc[0].1 + 2) / 3
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
