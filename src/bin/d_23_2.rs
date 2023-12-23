use std::{cell::RefCell, rc::Rc};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_23_1.txt");

struct Node {
    coords: (usize, usize),
    neighbours: Vec<(Rc<RefCell<Node>>, usize)>,
}

const TILES: [char; 5] = ['.', 'v', '^', '>', '<'];

fn main() {
    let grid = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let end_row = grid.len() - 1;
    let end_col = grid[end_row].iter().position(|&c| c == '.').unwrap();

    let nodes = grid
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.iter().enumerate().filter_map(move |(col_idx, c)| {
                if TILES.contains(c) {
                    Some(Rc::new(RefCell::new(Node {
                        coords: (col_idx, line_idx),
                        neighbours: vec![],
                    })))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    for node in &nodes {
        let (x, y) = node.borrow().coords;
        // North
        if let Some(n_node) = nodes.iter().find(|n| {
            let n = n.borrow();
            n.coords.0 == x && n.coords.1 + 1 == y
        }) {
            node.borrow_mut().neighbours.push((n_node.clone(), 1));
        }
        // South
        if let Some(n_node) = nodes.iter().find(|n| {
            let n = n.borrow();
            n.coords.0 == x && n.coords.1 == y + 1
        }) {
            node.borrow_mut().neighbours.push((n_node.clone(), 1));
        }
        // East
        if let Some(n_node) = nodes.iter().find(|n| {
            let n = n.borrow();
            n.coords.0 == x + 1 && n.coords.1 == y
        }) {
            node.borrow_mut().neighbours.push((n_node.clone(), 1));
        }
        // West
        if let Some(n_node) = nodes.iter().find(|n| {
            let n = n.borrow();
            n.coords.0 + 1 == x && n.coords.1 == y
        }) {
            node.borrow_mut().neighbours.push((n_node.clone(), 1));
        }
    }

    // Now remove nodes which only have 2 neighbours
    for node in nodes
        .iter()
        .filter(|node| node.borrow().neighbours.len() == 2)
    {
        {
            let node = node.borrow();
            let (n1, dist1) = &node.neighbours[0];
            let (n2, dist2) = &node.neighbours[1];

            let bridged_dist = dist1 + dist2;

            n1.borrow_mut()
                .neighbours
                .retain(|(n, _)| n.borrow().coords != node.coords);
            n1.borrow_mut().neighbours.push((n2.clone(), bridged_dist));
            n2.borrow_mut()
                .neighbours
                .retain(|(n, _)| n.borrow().coords != node.coords);
            n2.borrow_mut().neighbours.push((n1.clone(), bridged_dist));
        }
        node.borrow_mut().neighbours = vec![];
    }

    println!("Collapsed graph");

    let mut cur_path = vec![];
    let mut known_lengths = vec![];

    longest_path(
        &nodes[0],
        (end_col, end_row),
        &mut cur_path,
        0,
        &mut known_lengths,
    );

    println!(
        "Longest path is {} tiles long",
        known_lengths.iter().max().unwrap()
    );
}

fn longest_path(
    node: &Rc<RefCell<Node>>,
    end_pos: (usize, usize),
    cur_path: &mut Vec<(usize, usize)>,
    cur_length: usize,
    known_lengths: &mut Vec<usize>,
) {
    if cur_path.contains(&node.borrow().coords) {
        return;
    }
    if node.borrow().coords == end_pos {
        known_lengths.push(cur_length);
        return;
    }
    cur_path.push(node.borrow().coords);

    for (node, dist) in &node.borrow().neighbours {
        longest_path(node, end_pos, cur_path, cur_length + dist, known_lengths);
    }

    cur_path.pop();
}
