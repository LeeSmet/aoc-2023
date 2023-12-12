use std::collections::HashMap;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_12_1.txt");

#[derive(PartialEq, Eq, Hash)]
struct HKey {
    s: Vec<char>,
    c: usize,
    r: Vec<usize>,
}

struct Memory {
    m: HashMap<HKey, usize>,
}

fn main() {
    let mut m = Memory { m: HashMap::new() };

    let possibilities = INPUT_FILE
        .lines()
        .map(|line| {
            let (input, counts) = line.split_once(' ').unwrap();
            let mut inp = input.to_string();
            for _ in 0..4 {
                inp.push('?');
                inp.push_str(input);
            }
            let input = inp.chars().collect::<Vec<_>>();

            let counts = std::iter::repeat(counts)
                .take(5)
                .flat_map(|line| line.split(','))
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            m.solve(&input, 0, &counts)
        })
        .sum::<usize>();

    println!("Total valid row arrangements {possibilities}");
}

impl Memory {
    fn solve(&mut self, s: &[char], c: usize, r: &[usize]) -> usize {
        let hk = HKey {
            s: s.to_vec(),
            c,
            r: r.to_vec(),
        };
        if let Some(res) = self.m.get(&hk) {
            return *res;
        }

        if s.is_empty() {
            if c == 0 && r.is_empty() {
                self.m.insert(hk, 1);
                return 1;
            }
            if r.len() == 1 && c == r[0] {
                self.m.insert(hk, 1);
                return 1;
            }
            self.m.insert(hk, 0);
            return 0;
        }

        let pm = s.iter().filter(|&&c| c == '#' || c == '?').count();
        if pm + c < r.iter().sum() {
            self.m.insert(hk, 0);
            return 0;
        }
        if c > 0 && r.is_empty() {
            self.m.insert(hk, 0);
            return 0;
        }

        let mut p = 0;
        if s[0] == '.' && c > 0 && r[0] != c {
            self.m.insert(hk, 0);
            return 0;
        }
        if s[0] == '.' && c > 0 {
            p += self.solve(&s[1..], 0, &r[1..]);
        }
        if s[0] == '?' && c > 0 && c == r[0] {
            p += self.solve(&s[1..], 0, &r[1..]);
        }
        if s[0] == '#' || s[0] == '?' {
            p += self.solve(&s[1..], c + 1, r);
        }
        if (s[0] == '.' || s[0] == '?') && c == 0 {
            p += self.solve(&s[1..], 0, r);
        }
        self.m.insert(hk, p);
        p
    }
}
