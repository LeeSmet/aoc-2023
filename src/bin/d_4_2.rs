use std::collections::HashMap;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_4_1.txt");

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn extra_cards(&self) -> Vec<u32> {
        self.your_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .enumerate()
            .map(|(idx, _)| self.id + idx as u32 + 1)
            .collect()
    }

    /// Parse a single line. Format: `Card {card_id}: {winning_num}* | {your_nums}*`
    fn parse_line(line: &str) -> Self {
        let (card, nums) = line.split_once(':').unwrap();

        let (winning_nums, your_nums) = nums.split_once('|').unwrap();
        Card {
            id: card.strip_prefix("Card").unwrap().trim().parse().unwrap(),
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
    let mut cards = INPUT_FILE
        .lines()
        .map(|line| {
            let card = Card::parse_line(line);
            (card.id, (card, 1))
        })
        .collect::<HashMap<_, _>>();
    for id in 1..=cards.len() {
        let (card, count) = cards.get(&(id as u32)).unwrap();
        // Get rid of borrow on cards;
        let count = *count;
        let extras = card.extra_cards();
        for extra in extras {
            cards.get_mut(&extra).unwrap().1 += count;
        }
    }

    let total_cards = cards.values().map(|(_, count)| count).sum::<u32>();

    println!("Total scratchcards after playing: {total_cards}");
}
