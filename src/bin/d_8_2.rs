use std::{collections::HashMap, str::FromStr};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_8_1.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct NodeValue {
    value: [u8; 3],
}

impl NodeValue {
    fn suffix(&self, suffix: char) -> bool {
        self.value[2] == suffix as u8
    }
}

impl FromStr for NodeValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeValue {
            value: s.as_bytes().try_into().unwrap(),
        })
    }
}

struct Node {
    value: NodeValue,
    left: NodeValue,
    right: NodeValue,
}

impl Node {
    fn from_line(line: &str) -> Node {
        let (v, lr) = line.split_once('=').unwrap();
        let value = v.trim().parse().unwrap();

        let (l, r) = lr.trim().split_once(", ").unwrap();

        Node {
            value,
            left: l[1..].parse().unwrap(),
            right: r[..r.len() - 1].parse().unwrap(),
        }
    }
}

fn main() {
    let input = INPUT_FILE.lines().next().unwrap();
    let nodes = INPUT_FILE
        .lines()
        .skip(2)
        .map(Node::from_line)
        .map(|node| (node.value, (node.left, node.right)))
        .collect::<HashMap<_, _>>();

    let ring_count = nodes
        .iter()
        .filter_map(|n| if n.0.suffix('A') { Some(n.0) } else { None })
        .map(|node| {
            let mut n = *node;
            let mut steps = 0;
            'outer: loop {
                for c in input.chars() {
                    if n.suffix('Z') {
                        break 'outer;
                    }
                    steps += 1;
                    match c {
                        'L' => n = nodes[&n].0,
                        'R' => n = nodes[&n].1,
                        _ => unreachable!(),
                    }
                }
            }
            steps
        })
        .collect::<Vec<_>>();

    let mut steps = 1;

    for count in ring_count {
        steps = lcm(steps, count);
    }

    println!("It takes {steps} steps to reach the end");
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
