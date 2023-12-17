use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_17_1.txt");

fn main() {
    let grid: Vec<Vec<_>> = INPUT_FILE
        .lines()
        .map(|line| {
            line.chars()
                // Very poor conversion to digits
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let lowest_cost = find_path_cost(&grid);

    println!("Lowest path cost is {lowest_cost}");
}

fn find_path_cost(grid: &Vec<Vec<usize>>) -> usize {
    let mut known_distances = HashMap::new();
    let mut to_explore = BinaryHeap::from_iter([Reverse((0, (0, 0, (0, 0))))]);

    let target_x = grid[0].len() - 1;
    let target_y = grid.len() - 1;

    while let Some(Reverse((cost, (row, column, direction)))) = to_explore.pop() {
        if (column, row) == (target_x, target_y) {
            return cost;
        }
        if let Some(&known_cost) = known_distances.get(&(row, column, direction)) {
            if known_cost < cost {
                continue;
            }
        }
        for (row_change, column_change) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // Ignore the current direction as it should be fully explored, also don't do a full turn.
            if direction == (row_change, column_change)
                || direction == (-row_change, -column_change)
            {
                continue;
            }
            let mut new_cost = cost;
            // Can't start at three here as then we don't count the cost of those tiles.
            // We could if we first count those costs manually but lets just add an extra if to
            // skip these fields furhter down
            for i in 0..10 {
                let target_row = row as isize + row_change * (i + 1);
                let target_column = column as isize + column_change * (i + 1);

                if target_row < 0
                    || target_row as usize >= grid.len()
                    || target_column < 0
                    || target_column as usize >= grid[0].len()
                {
                    // Fell of the board
                    continue;
                }

                new_cost += grid[target_row as usize][target_column as usize];

                // Don't add paths in the to explore queue if we haven't made 4 steps in a single
                // direction yet.
                if i < 3 {
                    continue;
                }

                let existing_cost = known_distances
                    .entry((
                        target_row as usize,
                        target_column as usize,
                        (row_change, column_change),
                    ))
                    .or_insert(usize::MAX);
                if new_cost < *existing_cost {
                    *existing_cost = new_cost;
                    to_explore.push(Reverse((
                        new_cost,
                        (
                            target_row as usize,
                            target_column as usize,
                            (row_change, column_change),
                        ),
                    )));
                }
            }
        }
    }

    // We'll always get an answer
    unreachable!();
}
