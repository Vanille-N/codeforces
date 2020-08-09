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
    let t = scan.next::<u64>();
    for _ in 0..t {
        let n = scan.next::<u64>();
        let w = (0..n).map(|_| scan.next::<u64>()).collect::<Vec<_>>();
        println!("{}", max_nb_teams(w));
    }
}

fn max_nb_teams(w: Vec<u64>) -> u64 {
    use std::collections::{HashMap, HashSet};
    let mut teams: HashMap<u64, (u64, HashSet<u64>)> = HashMap::new();
    for i in 0..w.len() {
        for j in 0..i {
            let t = w[i] + w[j];
            if let Some((n, used)) = teams.get_mut(&t) {
                if !used.contains(&(i as u64)) && !used.contains(&(j as u64)) {
                    *n += 1;
                    used.insert(i as u64);
                    used.insert(j as u64);
                }
            } else {
                let mut used = HashSet::new();
                used.insert(i as u64);
                used.insert(j as u64);
                teams.insert(t, (1, used));
            }
        }
    }
    teams.into_iter().map(|(_, (v, _))| v).max().unwrap_or(0)
}
