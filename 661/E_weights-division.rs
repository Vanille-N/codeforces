use std::io::stdin;
use std::collections::HashSet;

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
        let nb_vert = scan.next::<u64>();
        let s = scan.next::<u64>();
        let vertices = (0..nb_vert).map(|_| {
            let v = scan.next::<u64>();
            let u = scan.next::<u64>();
            let w = scan.next::<u64>();
            (v, u, w)
        }).collect::<Vec<_>>();
        let n = {
            let mut n = 0;
            for &(u, v, _) in &vertices {
                n = n.max(u).max(v);
            }
            n
        };
        let usage = calc_usage(n, vertices);
        println!("{}", min_moves(s, usage));
    }
}

fn calc_usage(n: u64, vertices: Vec<(u64, u64, u64)>) -> Vec<(u64, u64, u64)> {
    let mut nodes = vec![vec![]; n as usize];
    for &(u, v, w) in &vertices {
        nodes[(u-1) as usize].push((v-1, w));
        nodes[(v-1) as usize].push((u-1, w));
    }

    let mut seen = HashSet::new();
    let mut passage = Vec::new();
    let _ = explore_tree(0, &mut seen, &nodes, &mut passage);
    passage
}

fn explore_tree(pos: u64, seen: &mut HashSet<u64>, nodes: &Vec<Vec<(u64, u64)>>, passage: &mut Vec<(u64, u64, u64)>) -> u64 {
    let mut tot = 1;
    for &(n, w) in nodes[pos as usize].iter() {
        if !seen.contains(&n) {
            seen.insert(n);
            let cnt = explore_tree(n, seen, nodes, passage);
            passage.push((cnt*w, cnt, w));
            tot += cnt;
        }
    }
    tot
}

fn min_moves(s: u64, usage: Vec<(u64, u64, u64)>) -> u64 {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for (p, cnt, w) in usage {
        sum += p;
        heap.push((p, cnt, w));
    }
    let mut nb = 0;
    while s < sum {
        let (p, cnt, w) = heap.pop().unwrap();
        nb += 1;
        let new_w = w / 2;
        let new_p = cnt * new_w;
        sum -= p - new_p;
        heap.push((new_p, cnt, new_w));
    }
    nb
}
