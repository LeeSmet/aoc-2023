use std::collections::HashSet;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_23_1.txt");

fn main() {
    let grid = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let end_row = grid.len() - 1;
    let end_col = grid[end_row].iter().position(|&c| c == '.').unwrap();

    let mut cur_path = HashSet::new();
    let mut known_lengths = HashSet::new();

    longest_path(
        &grid,
        (0, 1),
        (end_row, end_col),
        &mut cur_path,
        &mut known_lengths,
    );

    println!(
        "Longest path is {} tiles long",
        known_lengths.iter().max().unwrap()
    );
}

fn longest_path(
    grid: &Vec<Vec<char>>,
    cur_pos: (usize, usize),
    end_pos: (usize, usize),
    cur_path: &mut HashSet<(usize, usize)>,
    known_lengths: &mut HashSet<usize>,
) {
    if grid[cur_pos.0][cur_pos.1] == '#' {
        return;
    }
    if cur_path.contains(&cur_pos) {
        return;
    }
    if cur_pos == end_pos {
        known_lengths.insert(cur_path.len());
        return;
    }
    cur_path.insert(cur_pos);

    match grid[cur_pos.0][cur_pos.1] {
        '>' => {
            longest_path(
                grid,
                (cur_pos.0, cur_pos.1 + 1),
                end_pos,
                cur_path,
                known_lengths,
            );
        }
        '<' => {
            longest_path(
                grid,
                (cur_pos.0, cur_pos.1 - 1),
                end_pos,
                cur_path,
                known_lengths,
            );
        }
        '^' => {
            longest_path(
                grid,
                (cur_pos.0 - 1, cur_pos.1),
                end_pos,
                cur_path,
                known_lengths,
            );
        }
        'v' => {
            longest_path(
                grid,
                (cur_pos.0 + 1, cur_pos.1),
                end_pos,
                cur_path,
                known_lengths,
            );
        }
        '.' => {
            if cur_pos.1 > 0 {
                longest_path(
                    grid,
                    (cur_pos.0, cur_pos.1 - 1),
                    end_pos,
                    cur_path,
                    known_lengths,
                );
            }
            if cur_pos.1 < grid[0].len() - 1 {
                longest_path(
                    grid,
                    (cur_pos.0, cur_pos.1 + 1),
                    end_pos,
                    cur_path,
                    known_lengths,
                );
            }
            if cur_pos.0 > 0 {
                longest_path(
                    grid,
                    (cur_pos.0 - 1, cur_pos.1),
                    end_pos,
                    cur_path,
                    known_lengths,
                );
            }
            if cur_pos.0 < grid.len() - 1 {
                longest_path(
                    grid,
                    (cur_pos.0 + 1, cur_pos.1),
                    end_pos,
                    cur_path,
                    known_lengths,
                );
            }
        }
        _ => unreachable!(),
    }

    cur_path.remove(&cur_pos);
}
