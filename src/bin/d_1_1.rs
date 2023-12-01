/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_1_1.txt");

fn main() {
    let total: u32 = INPUT_FILE
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter(char::is_ascii_digit);
            let first = nums.next();
            let last = nums.last();

            match (first, last) {
                (Some(first), Some(last)) => {
                    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
                }
                (Some(first), None) => first.to_digit(10).unwrap() * 11,
                (None, None) => 0,
                (None, Some(_)) => unreachable!(),
            }
        })
        .sum();

    println!("Total calibration values sum is {total}");
}
