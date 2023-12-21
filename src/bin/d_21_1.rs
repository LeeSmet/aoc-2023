use std::collections::HashSet;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_21_1.txt");

fn main() {
    let grid: Vec<Vec<_>> = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (start_row, start_col) = grid
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| row.iter().enumerate().zip(std::iter::repeat(row_idx)))
        .find_map(|((col, &v), row)| if v == 'S' { Some((row, col)) } else { None })
        .unwrap();

    let mut possible_fields = HashSet::new();

    possible_fields.insert((start_row, start_col));

    for _ in 0..64 {
        let mut new_fields = HashSet::new();
        for field in &possible_fields {
            if field.0 > 0 && grid[field.0 - 1][field.1] != '#' {
                new_fields.insert((field.0 - 1, field.1));
            }
            if field.1 > 0 && grid[field.0][field.1 - 1] != '#' {
                new_fields.insert((field.0, field.1 - 1));
            }
            if field.0 < grid.len() - 1 && grid[field.0 + 1][field.1] != '#' {
                new_fields.insert((field.0 + 1, field.1));
            }
            if field.1 < grid[0].len() - 1 && grid[field.0][field.1 + 1] != '#' {
                new_fields.insert((field.0, field.1 + 1));
            }
        }

        possible_fields = new_fields;
    }

    println!(
        "There are {} possible fields to visist",
        possible_fields.len()
    );
}
