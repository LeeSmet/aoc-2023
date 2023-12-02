/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_1_1.txt");

/// Possible representation of digits and their values
const DIGITS: [(&str, u32); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() {
    let total: u32 = INPUT_FILE
        .lines()
        .map(|mut line| {
            let mut vals = vec![];
            while !line.is_empty() {
                for (word, value) in DIGITS {
                    if line.strip_prefix(word).is_some() {
                        vals.push(value);
                        break;
                    }
                }
                line = &line[1..];
            }

            if vals.is_empty() {
                return 0;
            }
            vals.first().unwrap() * 10 + vals.last().unwrap()
        })
        .sum();

    println!("Total calibration values sum is {total}");
}
