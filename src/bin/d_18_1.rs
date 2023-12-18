/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_18_1.txt");

fn main() {
    let mut coords = vec![(0, 0)];

    for (direction, length) in INPUT_FILE.lines().map(|line| {
        let mut parts = line.split_whitespace();
        (
            parts.next().unwrap(),
            // parse as isize to avoid casts later as coords can be negative
            parts.next().unwrap().parse::<isize>().unwrap(),
        )
    }) {
        let mut last_coords = coords[coords.len() - 1];

        for _ in 0..length {
            let new_coords = match direction {
                "U" => (last_coords.0, last_coords.1 + 1),
                "D" => (last_coords.0, last_coords.1 - 1),
                "R" => (last_coords.0 + 1, last_coords.1),
                "L" => (last_coords.0 - 1, last_coords.1),
                _ => unreachable!(),
            };

            coords.push(new_coords);
            last_coords = new_coords;
        }
    }

    // Determine grid size by finding points farthest away from eachother on row/column lines
    let (min_x, max_x, min_y, max_y) =
        coords
            .iter()
            .copied()
            .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), (x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            });

    let columns = (max_x - min_x + 1) as usize;
    let rows = (max_y - min_y + 1) as usize;

    let mut grid = vec![vec!["."; columns]; rows];

    // Add edges to grid.
    for coord in coords {
        grid[(coord.1 - min_y) as usize][(coord.0 - min_x) as usize] = "#";
    }

    // Fill in interior
    let mut new_grid = grid.clone();
    for (row_idx, row) in new_grid.iter_mut().enumerate() {
        let mut interior = false;
        for (col_idx, field) in row.iter_mut().enumerate() {
            if *field == "#" {
                if ((row_idx > 0 && row_idx < rows - 1)
                    && (grid[row_idx - 1][col_idx] == "#" && grid[row_idx + 1][col_idx] == "#"))
                    || ((col_idx < columns - 1 && row_idx < rows - 1)
                        && (grid[row_idx][col_idx + 1] == "#" && grid[row_idx + 1][col_idx] == "#"))
                    || ((row_idx < rows - 1 && col_idx > 0)
                        && (grid[row_idx][col_idx - 1] == "#" && grid[row_idx + 1][col_idx] == "#"))
                {
                    interior = !interior;
                }
            } else if interior {
                *field = "#";
            }
        }
    }

    let area = new_grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x != ".")
        .count();

    println!("Area dug out is {area}");
}
