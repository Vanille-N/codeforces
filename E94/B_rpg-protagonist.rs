
fn main() {
    let mut scan = Scanner::default();
    let t: u64 = scan.next();
    for _ in 0..t {
        let capa: (u64, u64) = (scan.next(), scan.next());
        let cnt: (u64, u64) = (scan.next(), scan.next());
        let wht: (u64, u64) = (scan.next(), scan.next());
        println!("{}", max_weapons(capa, cnt, wht));
    }
}

fn max_weapons(capa: (u64, u64), cnt: (u64, u64), wht: (u64, u64)) -> u64 {
    let (cnt, wht) = {
        if wht.0 > wht.1 {
            ((cnt.1, cnt.0), (wht.1, wht.0))
        } else {
            (cnt, wht)
        }
    };
    let mut best = 0;
    for limit in 0..cnt.0 {
        let distrib_1 = {
            let self_light = (capa.0 / wht.0).min(cnt.0).min(limit);
            let self_heavy = ((capa.0 - self_light * wht.0) / wht.1).min(cnt.1);
            let other_light = (capa.1 / wht.0).min(cnt.0 - self_light);
            let other_heavy = ((capa.1 - other_light * wht.0) / wht.1).min(cnt.1 - self_heavy);
            // println!("{} s + {} w + {} s + {} w", self_light, self_heavy, other_light, other_heavy);
            self_heavy + self_light + other_heavy + other_light
        };
        let distrib_2 = {
            let self_light = (capa.1 / wht.0).min(cnt.0).min(limit);
            let self_heavy = ((capa.1 - self_light * wht.0) / wht.1).min(cnt.1);
            let other_light = (capa.0 / wht.0).min(cnt.0 - self_light);
            let other_heavy = ((capa.0 - other_light * wht.0) / wht.1).min(cnt.1 - self_heavy);
            // println!("{} s + {} w + {} s + {} w", self_light, self_heavy, other_light, other_heavy);
            self_heavy + self_light + other_heavy + other_light
        };
        // println!("--> {}", distrib);
        best = best.max(distrib_1).max(distrib_2);
    }
    best
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
