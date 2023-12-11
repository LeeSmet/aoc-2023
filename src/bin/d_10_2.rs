/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_10_1.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Connection {
    NorthEast,
    NorthSouth,
    NorthWest,
    EastWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

type Coords = (usize, usize);

impl Connection {
    fn connected_tiles(&self, i: usize, j: usize) -> Option<(Coords, Coords)> {
        match self {
            Connection::NorthEast => Some(((i - 1, j), (i, j + 1))),
            Connection::NorthSouth => Some(((i - 1, j), (i + 1, j))),
            Connection::NorthWest => Some(((i - 1, j), (i, j - 1))),
            Connection::EastWest => Some(((i, j + 1), (i, j - 1))),
            Connection::SouthEast => Some(((i, j + 1), (i + 1, j))),
            Connection::SouthWest => Some(((i + 1, j), (i, j - 1))),
            Connection::Ground => None,
            Connection::Start => None,
        }
    }

    fn connected_south(&self) -> bool {
        matches!(
            self,
            Connection::SouthWest | Connection::SouthEast | Connection::NorthSouth
        )
    }

    fn connected_west(&self) -> bool {
        matches!(
            self,
            Connection::SouthWest | Connection::NorthWest | Connection::EastWest
        )
    }
}

fn main() {
    let field = INPUT_FILE
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Connection::Start,
                    '.' => Connection::Ground,
                    '|' => Connection::NorthSouth,
                    '-' => Connection::EastWest,
                    'L' => Connection::NorthEast,
                    'J' => Connection::NorthWest,
                    '7' => Connection::SouthWest,
                    'F' => Connection::SouthEast,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find start
    let (start_row, start_col) = field
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &con)| con == Connection::Start)
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    let mut prev = (start_row, start_col);
    let mut elem = if start_row > 0 && field[start_row - 1][start_col].connected_south() {
        (start_row - 1, start_col)
    } else if start_col < field[start_row].len() && field[start_row][start_col + 1].connected_west()
    {
        (start_row, start_col + 1)
    } else {
        (start_row, start_col - 1)
    };

    let mut elements = vec![(prev, Connection::Start), (elem, field[elem.0][elem.1])];

    while let Some((coords1, coords2)) = field[elem.0][elem.1].connected_tiles(elem.0, elem.1) {
        if coords1 != prev {
            prev = elem;
            elem = coords1;
        } else {
            prev = elem;
            elem = coords2
        }
        elements.push((elem, field[elem.0][elem.1]));
    }

    let mut enclosed_fields = 0;
    for (row, line) in field.iter().enumerate() {
        for col in 0..line.len() {
            if elements.iter().any(|((i, j), _)| i == &row && j == &col) {
                continue;
            }

            let mut row_elements = elements
                .iter()
                .filter(|((i, _), _)| row == *i)
                .copied()
                .collect::<Vec<_>>();
            row_elements.sort_by(|a, b| a.0 .1.cmp(&b.0 .1));

            let (before, _) = row_elements.into_iter().fold(
                (0, Connection::Ground),
                |(before, lc), ((_, j), c)| {
                    if (c == Connection::NorthSouth
                        || c == Connection::NorthWest && lc == Connection::SouthEast
                        || c == Connection::SouthWest && lc == Connection::NorthEast)
                        && j < col
                    {
                        (before + 1, c)
                    } else if c == Connection::EastWest {
                        (before, lc)
                    } else {
                        (before, c)
                    }
                },
            );

            if before % 2 == 1 {
                enclosed_fields += 1;
            }
        }
    }
    println!("Enclosed fields {enclosed_fields}");
}
