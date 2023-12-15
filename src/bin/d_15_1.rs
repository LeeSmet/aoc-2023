/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_15_1.txt");

fn main() {
    let hash_sum = INPUT_FILE
        .split(',')
        .map(str::trim)
        .map(hash)
        .sum::<usize>();
    println!("Total hash sum is {hash_sum}");
}

// Technically this returns a u8 since result is clamped to [0,255] but we use usize to avoid
// casting everywhere.
fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (((c as usize) + acc) * 17) % 256)
}
