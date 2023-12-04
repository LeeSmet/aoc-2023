/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_4_1.txt");

struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let matches = self
            .your_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
        if matches == 0 {
            0
        } else {
            2u32.pow(matches as u32 - 1)
        }
    }

    /// Parse a single line. Format: `Card {card_id}: {winning_num}* | {your_nums}*`
    fn parse_line(line: &str) -> Self {
        let (_, nums) = line.split_once(':').unwrap();

        let (winning_nums, your_nums) = nums.split_once('|').unwrap();
        Card {
            winning_numbers: winning_nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
            your_numbers: your_nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}
fn main() {
    let score: u32 = INPUT_FILE
        .lines()
        .map(|line| Card::parse_line(line).score())
        .sum();

    println!("Sum of card scores is {score}");
}
