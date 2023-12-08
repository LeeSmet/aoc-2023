use std::{collections::HashMap, str::FromStr};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_8_1.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct NodeValue {
    value: [u8; 3],
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

    let mut current = "AAA".parse::<NodeValue>().unwrap();
    let mut steps = 0;
    let end = NodeValue::from_str("ZZZ").unwrap();

    'outer: loop {
        for c in input.chars() {
            if current == end {
                break 'outer;
            }
            steps += 1;
            match c {
                'L' => current = nodes[&current].0,
                'R' => current = nodes[&current].1,
                _ => unreachable!(),
            }
        }
    }

    println!("It takes {steps} steps to reach the end");
}
