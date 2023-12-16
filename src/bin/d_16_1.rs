use std::collections::HashSet;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_16_1.txt");

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

fn main() {
    let grid: Vec<Vec<_>> = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut beams: Vec<(isize, isize, usize)> = vec![(0, -1, 1)];

    let mut fields_visited = HashSet::new();
    let mut explored_beams = HashSet::new();

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    loop {
        if beams.is_empty() {
            break;
        }
        let mut beam = beams.remove(0);

        loop {
            let (i, j) = match beam {
                (i, j, NORTH) => (i - 1, j),
                (i, j, EAST) => (i, j + 1),
                (i, j, SOUTH) => (i + 1, j),
                (i, j, WEST) => (i, j - 1),
                _ => unreachable!(),
            };

            if i < 0 || i >= rows || j < 0 || j >= cols {
                break;
            }

            // Remember we visited the field now that we know it is in bounds.
            fields_visited.insert((i, j));

            let new_dir = match (beam.2, grid[i as usize][j as usize]) {
                // Mirror 1
                (NORTH, '\\') => WEST,
                (EAST, '\\') => SOUTH,
                (SOUTH, '\\') => EAST,
                (WEST, '\\') => NORTH,
                // Mirror 2
                (NORTH, '/') => EAST,
                (EAST, '/') => NORTH,
                (SOUTH, '/') => WEST,
                (WEST, '/') => SOUTH,
                // - splitter
                (NORTH, '-') => {
                    if !explored_beams.contains(&(i, j, WEST)) {
                        beams.push((i, j, WEST));
                    }
                    EAST
                }
                (SOUTH, '-') => {
                    if !explored_beams.contains(&(i, j, WEST)) {
                        beams.push((i, j, WEST));
                    }
                    EAST
                }
                // | spliter
                (EAST, '|') => {
                    if !explored_beams.contains(&(i, j, SOUTH)) {
                        beams.push((i, j, SOUTH));
                    }
                    NORTH
                }
                (WEST, '|') => {
                    if !explored_beams.contains(&(i, j, SOUTH)) {
                        beams.push((i, j, SOUTH));
                    }
                    NORTH
                }
                // Going anywere else does nothing, i.e. empty field or going straight into a
                // splitter on the pointy part.
                (x, _) => x,
            };

            if explored_beams.contains(&(i, j, new_dir)) {
                break;
            }
            explored_beams.insert((i, j, new_dir));
            beam = (i, j, new_dir)
        }
    }

    println!("Energiezed fields: {}", fields_visited.len());
}
