/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_6_1.txt");

fn main() {
    let mut input_lines = INPUT_FILE.lines();

    let time = input_lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input_lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    // First equality, the first integer valid must be bigger or equal to this
    let inf1 = ((time as f64 - ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.).ceil() as u64;
    // First equality, the last integer valid must be smaller or equal to this
    let inf2 = ((time as f64 + ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.).floor() as u64;

    let opts = inf2 + 1 - inf1;

    println!("There are {opts} possible ways to win the race");
}
