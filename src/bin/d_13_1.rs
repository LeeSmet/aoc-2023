/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_13_1.txt");

fn main() {
    let patterns = INPUT_FILE
        .lines()
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(Vec::from)
        .collect::<Vec<_>>();

    let score = patterns
        .iter()
        .map(|pattern| {
            // first find possible horizontal reflection
            let mut hor_ref = 0;
            for i in 0..pattern.len() - 1 {
                let mut eq = true;
                for j in 0..(i + 1).min(pattern.len() - i - 1) {
                    if pattern[i - j] != pattern[i + j + 1] {
                        eq = false;
                        break;
                    }
                }
                if eq {
                    hor_ref = i + 1;
                    break;
                }
            }

            // then find possible vertical reflection.
            let mut ver_ref = 0;
            for i in 0..pattern[0].len() - 1 {
                let mut eq = true;
                let scan_len = (i + 1).min(pattern[0].len() - i - 1);
                for row in pattern {
                    if row[i + 1 - scan_len..i + 1]
                        != row[i + 1..i + 1 + scan_len]
                            .chars()
                            .rev()
                            .collect::<String>()
                    {
                        eq = false;
                        break;
                    }
                }
                if eq {
                    ver_ref = i + 1;
                    break;
                }
            }

            ver_ref + 100 * hor_ref
        })
        .sum::<usize>();

    println!("Summarized score is {score}");
}
