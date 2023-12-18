/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_18_1.txt");

fn main() {
    let mut x = 0;
    let mut perimeter = 0;
    let mut area = 0;

    for (direction, length) in INPUT_FILE
        .lines()
        .map(|line| {
            let num = line.split_whitespace().nth(2).unwrap().trim();
            (
                ["R", "D", "L", "U"][num.chars().nth(7).unwrap().to_digit(10).unwrap() as usize],
                // parse as isize to avoid casts later as coords can be negative
                isize::from_str_radix(&num[2..7], 16).unwrap(),
            )
        })
        // Rev here since forward iteration gives us a negative area, i.e. it happens that our
        // input is counter clockwise
        .rev()
    {
        let (dx, dy) = match direction {
            "U" => (0, length),
            "D" => (0, -length),
            "R" => (length, 0),
            "L" => (-length, 0),
            _ => unreachable!(),
        };
        x += dx;
        perimeter += length;
        area += x * dy;
    }

    println!("total area dug out: {}", area + perimeter / 2 + 1);
}
