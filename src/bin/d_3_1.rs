/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_3_1.txt");

struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

struct Symbol {
    row: usize,
    col: usize,
}

fn main() {
    let mut numbers = vec![];
    let mut symbols = vec![];

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

            symbols.push(Symbol { row, col });
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
    for number in &numbers {
        for symbol in &symbols {
            if symbol.row >= number.row.saturating_sub(1)
                && symbol.row <= number.row + 1
                && symbol.col >= number.start.saturating_sub(1)
                && symbol.col <= number.end + 1
            {
                sum += number.value;
                break;
            }
        }
    }

    println!("Sum of engine part numbers is {sum}");
}
