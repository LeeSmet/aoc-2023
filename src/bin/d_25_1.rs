use std::collections::{HashMap, HashSet, VecDeque};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_25_1.txt");

fn main() {
    let mut nodes = Vec::new();
    let mut edge_map = HashMap::<_, HashSet<_>>::new();
    for (node, edges) in INPUT_FILE.lines().map(|l| l.split_once(':').unwrap()) {
        if !nodes.contains(&node.to_string()) {
            nodes.push(node.trim().to_string());
        }
        for edge in edges.split_whitespace().map(str::trim) {
            if !nodes.contains(&edge.to_string()) {
                nodes.push(edge.to_string());
            }
            edge_map
                .entry(edge.to_string())
                .or_default()
                .insert(node.to_string());
            edge_map
                .entry(node.to_string())
                .or_default()
                .insert(edge.to_string());
        }
    }

    // Use Girvan-Newman to cut 3 paths
    for _ in 0..3 {
        let mut total_edge_visists = HashMap::<_, u64>::new();
        for node in &nodes {
            for (edge, count) in find_paths(node, &nodes, &edge_map) {
                if edge.0 < edge.1 {
                    *total_edge_visists.entry(edge).or_default() += count;
                } else {
                    *total_edge_visists.entry((edge.1, edge.0)).or_default() += count;
                }
            }
        }

        let e = total_edge_visists
            .into_iter()
            .max_by_key(|(_, c)| *c)
            .map(|(e, _)| e)
            .unwrap();
        edge_map.get_mut(&e.0).unwrap().remove(&e.1);
        edge_map.get_mut(&e.1).unwrap().remove(&e.0);
    }

    // Assume we have 2 distinct sets at this point. Find all connected nodes in p1.
    let mut s1 = HashSet::new();
    let mut to_explore = VecDeque::new();
    to_explore.push_back(nodes[0].clone());

    while let Some(node) = to_explore.pop_front() {
        s1.insert(node.clone());
        for connected in edge_map.get(&node).unwrap() {
            if s1.insert(connected.clone()) {
                to_explore.push_back(connected.clone());
            }
        }
    }

    println!(
        "Set sizes {} and {}, multiplied value is {}",
        s1.len(),
        nodes.len() - s1.len(),
        s1.len() * (nodes.len() - s1.len())
    );
}

fn find_paths(
    start_node: &str,
    nodes: &[String],
    edges: &HashMap<String, HashSet<String>>,
) -> HashMap<(String, String), u64> {
    let mut e_count = HashMap::new();

    let mut path = vec![(usize::MAX, HashSet::new()); nodes.len()];
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();

    for next_node in edges.get(&start_node.to_string()).unwrap() {
        to_visit.push_back((next_node.clone(), start_node.to_string(), HashSet::new()));
    }

    while let Some((n, prev, mut p)) = to_visit.pop_front() {
        p.insert((prev.clone(), n.clone()));
        let pos = nodes.iter().position(|p| p == &n).unwrap();
        if path[pos].0 > p.len() {
            path[pos] = (p.len(), p.clone());
        }
        for edge in edges.get(&n).unwrap() {
            if visited.insert(edge.clone()) {
                to_visit.push_back((edge.clone(), n.clone(), p.clone()));
            }
        }
    }

    for edge in path.into_iter().flat_map(|(_, s)| s) {
        *e_count.entry(edge).or_default() += 1;
    }

    e_count
}
