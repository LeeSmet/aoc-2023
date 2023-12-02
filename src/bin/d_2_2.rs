/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_2_1.txt");

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

struct Game {
    draws: Vec<Draw>,
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let games = INPUT_FILE.lines().map(parse_line).collect::<Vec<_>>();
    let sum = games
        .into_iter()
        .map(|game| {
            let needed_draw = game
                .draws
                .into_iter()
                .reduce(|acc, draw| Draw {
                    red: acc.red.max(draw.red),
                    green: acc.green.max(draw.green),
                    blue: acc.blue.max(draw.blue),
                })
                .unwrap();

            needed_draw.red * needed_draw.green * needed_draw.blue
        })
        .sum::<u32>();

    println!("Sum of power of all games: {sum}");
}

/// Parse a single line
fn parse_line(line: &str) -> Game {
    let line = line.strip_prefix("Game ").unwrap();
    let (_, line) = line.split_once(':').unwrap();
    let mut draws = vec![];
    for d in line.split(";") {
        // draw format is {num} {color}
        let mut tokens = d.split_whitespace();
        let mut draw = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        loop {
            let amount = tokens.next();
            let color = tokens.next();

            match (amount, color) {
                (Some(amount), Some(color)) => match color.trim_end_matches(',') {
                    RED => draw.red = amount.parse().unwrap(),
                    GREEN => draw.green = amount.parse().unwrap(),
                    BLUE => draw.blue = amount.parse().unwrap(),
                    x => panic!("Unrecognized color {x}"),
                },
                (None, None) => break,
                _ => unreachable!(),
            }
        }
        draws.push(draw);
    }

    Game { draws }
}
