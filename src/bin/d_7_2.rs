use std::collections::HashMap;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_7_1.txt");

#[derive(PartialEq, Eq)]
struct Draw {
    cards: [Card; 5],
    bid: usize,
}

impl Ord for Draw {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.draw_type().cmp(&other.draw_type()) {
            std::cmp::Ordering::Equal => {
                for (x, y) in self.cards.iter().zip(other.cards.iter()) {
                    if x != y {
                        return x.cmp(y);
                    }
                }
                std::cmp::Ordering::Equal
            }
            x => x,
        }
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum DrawType {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Draw {
    /// Parses a draw from a line
    fn from_line(line: &str) -> Draw {
        let cards = std::array::from_fn(|idx| match line.as_bytes()[idx] {
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::J,
            b'T' => Card::Ten,
            b'9' => Card::Nine,
            b'8' => Card::Eight,
            b'7' => Card::Seven,
            b'6' => Card::Six,
            b'5' => Card::Five,
            b'4' => Card::Four,
            b'3' => Card::Three,
            b'2' => Card::Two,
            x => panic!("Invalid character {x}"),
        });

        Draw {
            cards,
            bid: line[6..].parse().unwrap(),
        }
    }

    /// The type of draw of the hand
    fn draw_type(&self) -> DrawType {
        let mut card_count = HashMap::<_, usize>::with_capacity(5);
        for card in self.cards {
            *card_count.entry(card).or_default() += 1;
        }

        let jokers = card_count.remove(&Card::J).unwrap_or(0);

        match card_count.len() {
            5 => DrawType::High,
            4 => DrawType::Pair,
            3 => match card_count.values().max().unwrap() + jokers {
                3 => DrawType::ThreeOfAKind,
                2 => DrawType::TwoPair,
                _ => unreachable!(),
            },
            2 => match card_count.values().max().unwrap() + jokers {
                4 => DrawType::FourOfAKind,
                3 => DrawType::FullHouse,
                _ => unreachable!(),
            },
            1 => DrawType::FiveOfAKind,
            // 5 jokers
            0 => DrawType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut draws = INPUT_FILE.lines().map(Draw::from_line).collect::<Vec<_>>();
    draws.sort();

    let sum: usize = draws
        .into_iter()
        .enumerate()
        .map(|(idx, draw)| draw.bid * (idx + 1))
        .sum();

    println!("Sum of winnings is {sum}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ord() {
        assert!(super::Card::K > super::Card::Q);
    }
}
