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
        .find_map(|((col, &v), row)| {
            if v == 'S' {
                Some((row as isize, col as isize))
            } else {
                None
            }
        })
        .unwrap();

    let m = 26501365 % grid.len();

    let mut seen_states = vec![];

    for run in [m, m + grid.len(), m + grid.len() * 2] {
        let mut next_queue = vec![(start_row, start_col)];

        for _ in 0..run {
            let mut current = next_queue.clone();
            let mut visited = next_queue.into_iter().collect::<HashSet<_>>();

            next_queue = vec![];

            while let Some((x, y)) = current.pop() {
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let new_x = x + dx;
                    let new_y = y + dy;

                    let mut x_coord = new_x % (grid[0].len() as isize);
                    if x_coord < 0 {
                        x_coord += grid[0].len() as isize;
                    }
                    let mut y_coord = new_y % (grid.len() as isize);
                    if y_coord < 0 {
                        y_coord += grid.len() as isize;
                    }

                    if grid[y_coord as usize][x_coord as usize] != '#'
                        && visited.insert((new_x, new_y))
                    {
                        next_queue.push((new_x, new_y));
                    }
                }
            }
        }

        seen_states.push(next_queue.len());
    }

    let m = (seen_states[1] - seen_states[0]) as isize;
    let n = (seen_states[2] - seen_states[1]) as isize;
    let a = (n - m) / 2;
    let b = m - 3 * a;
    let c = seen_states[0] as isize - b - a;

    let ceiling = ((26501365 + grid.len() - 1) / grid.len()) as isize;

    let answer = a * ceiling.pow(2) + b * ceiling + c;

    println!("There are {answer} possible fields to visist");
}
