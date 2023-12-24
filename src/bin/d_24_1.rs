/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_24_1.txt");

const LEAST: f64 = 200000000000000.;
const MOST: f64 = 400000000000000.;

/// Representation of a line in the XY plane as y = mx + c
struct Line {
    // Original x coord
    x: f64,
    m: f64,
    c: f64,
    // Velocity of x is negative
    x_descending: bool,
}

impl Line {
    /// Check if lines intersect in the future of both lines.
    fn intersect(&self, other: &Line) -> Option<(f64, f64)> {
        if self.m == other.m && self.c != other.c {
            return None;
        }
        // m1x + c1 = m2x + c2 => (m1x - m2x) = c2 - c1 => x = (c2 - c1) / (m1 - m2)
        let x = (other.c - self.c) / (self.m - other.m);
        if (x < self.x && !self.x_descending)
            || (x > self.x && self.x_descending)
            || (x < other.x && !other.x_descending)
            || (x > other.x && other.x_descending)
        {
            return None;
        }
        let y = self.m * x + self.c;
        Some((x, y))
    }
}

fn main() {
    let lines = INPUT_FILE
        .lines()
        .map(|line| {
            let (coords, velocity) = line.split_once('@').unwrap();
            let mut coords = coords
                .split(',')
                .map(str::trim)
                .map(|v| v.parse::<f64>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            let mut velocity = velocity
                .split(',')
                .map(str::trim)
                .map(|v| v.parse::<f64>().unwrap());
            let vx = velocity.next().unwrap();
            let vy = velocity.next().unwrap();

            let m = vy / vx;
            // y = mx + c -> c = y - mx
            let c = y - m * x;

            Line {
                x,
                m,
                c,
                x_descending: vx < 0.,
            }
        })
        .collect::<Vec<_>>();

    let intersections_in_block = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| std::iter::repeat(line).zip(lines.iter().skip(i + 1)))
        .filter_map(|(line1, line2)| {
            if let Some((x, y)) = line1.intersect(line2) {
                if (LEAST..=MOST).contains(&x) && (LEAST..=MOST).contains(&y) {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .count();

    println!("There are {intersections_in_block} valid intersections");
}
