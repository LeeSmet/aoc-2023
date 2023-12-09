/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_9_1.txt");

fn main() {
    let mut new_vals = vec![];
    for line in INPUT_FILE.lines().map(str::split_whitespace) {
        let start_range = line
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let mut ranges = vec![start_range];

        loop {
            let curr_range = &ranges[ranges.len() - 1];
            if curr_range.is_empty() || curr_range.iter().all(|&i| i == 0) {
                break;
            }
            let new_range = curr_range.windows(2).map(|w| w[1] - w[0]).collect();
            ranges.push(new_range);
        }

        let mut addr = 0;

        ranges.reverse();

        for range in &ranges {
            addr = range[0] - addr;
        }

        new_vals.push(addr);
    }

    let sum = new_vals.into_iter().sum::<isize>();

    println!("Sum of new values is {sum}");
}
