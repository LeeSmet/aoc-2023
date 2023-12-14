use std::collections::HashMap;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_14_1.txt");

fn main() {
    let mut field = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut seen_fields = HashMap::new();

    let mut i = 0;
    loop {
        // Tilt north
        for i in 1..field.len() {
            for j in (0..i).rev() {
                for k in 0..field[i].len() {
                    if field[j][k] == '.' && field[j + 1][k] == 'O' {
                        field[j][k] = 'O';
                        field[j + 1][k] = '.';
                    }
                }
            }
        }

        // Tilt west
        for i in 1..field[0].len() {
            for j in (0..i).rev() {
                for row in &mut field {
                    if row[j] == '.' && row[j + 1] == 'O' {
                        row[j] = 'O';
                        row[j + 1] = '.';
                    }
                }
            }
        }

        // Tilt south
        for i in (0..field.len() - 1).rev() {
            for j in i..field.len() - 1 {
                for k in 0..field[0].len() {
                    if field[j + 1][k] == '.' && field[j][k] == 'O' {
                        field[j + 1][k] = 'O';
                        field[j][k] = '.';
                    }
                }
            }
        }

        // Tilt east
        for i in (0..field[0].len() - 1).rev() {
            for j in i..field[0].len() - 1 {
                for row in &mut field {
                    if row[j + 1] == '.' && row[j] == 'O' {
                        row[j + 1] = 'O';
                        row[j] = '.';
                    }
                }
            }
        }

        if let Some(prev_seen_idx) = seen_fields.get(&field) {
            let cycle_duration = i - prev_seen_idx;
            let remainder = 1_000_000_000 - i - 1;
            let cycles_left = remainder / cycle_duration;
            i += cycle_duration * cycles_left;
        } else {
            seen_fields.insert(field.clone(), i);
        }
        i += 1;

        if i == 1_000_000_000 {
            break;
        }
    }

    let load = field
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (field.len() - i))
        .sum::<usize>();

    println!("Total load is {load}");
}
