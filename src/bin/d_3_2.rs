/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_3_1.txt");

struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

struct Gear {
    row: usize,
    col: usize,
}

fn main() {
    let mut numbers = vec![];
    let mut gears = vec![];

    for (row, line) in INPUT_FILE.lines().enumerate() {
        let mut acc = 0;
        let mut num_start = None;
        for (col, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if num_start.is_none() {
                    num_start = Some(col);
                }

                acc = acc * 10 + char.to_string().parse::<u32>().unwrap();
                continue;
            }

            if let Some(ns) = num_start.take() {
                numbers.push(Number {
                    value: acc,
                    row,
                    start: ns,
                    end: col - 1,
                });
                acc = 0;
            }

            if char == '.' {
                continue;
            }

            gears.push(Gear { row, col });
        }

        // if number runs until end of line handle that here.
        if let Some(ns) = num_start.take() {
            numbers.push(Number {
                value: acc,
                row,
                start: ns,
                end: line.len() - 1,
            });
        }
    }

    let mut sum = 0;
    for gear in &gears {
        let mut adjacent_nums = vec![];
        for number in &numbers {
            if gear.row >= number.row.saturating_sub(1)
                && gear.row <= number.row + 1
                && gear.col >= number.start.saturating_sub(1)
                && gear.col <= number.end + 1
            {
                adjacent_nums.push(number.value);
            }
        }
        if adjacent_nums.len() == 2 {
            sum += adjacent_nums
                .into_iter()
                .reduce(|acc, item| acc * item)
                .unwrap();
        }
    }

    println!("Sum of engine part numbers is {sum}");
}
