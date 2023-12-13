/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_13_1.txt");

fn main() {
    let mut patterns = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(Vec::from)
        .collect::<Vec<_>>();

    let score = patterns
        .iter_mut()
        .map(|pattern| {
            let (_, ov, oh) = score(pattern, None, None);
            let rows = pattern.len();
            let cols = pattern[0].len();
            for i in 0..rows {
                for j in 0..cols {
                    let orig = pattern[i][j];
                    match orig {
                        '.' => pattern[i][j] = '#',
                        '#' => pattern[i][j] = '.',
                        _ => unreachable!(),
                    }
                    let (s, _, _) = score(
                        pattern,
                        if ov != 0 { Some(ov) } else { None },
                        if oh != 0 { Some(oh) } else { None },
                    );
                    if s != 0 {
                        return s;
                    }

                    pattern[i][j] = orig;
                }
            }

            unreachable!()
        })
        .sum::<usize>();

    println!("Summarized score is {score}");
}

fn score(
    pattern: &[Vec<char>],
    ignore_ov: Option<usize>,
    ignore_oh: Option<usize>,
) -> (usize, usize, usize) {
    // first find possible horizontal reflection
    let mut hor_ref = 0;
    for i in 0..pattern.len() - 1 {
        if let Some(ignore_oh) = ignore_oh {
            if ignore_oh == i + 1 {
                continue;
            };
        }
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
        if let Some(ignore_ov) = ignore_ov {
            if ignore_ov == i + 1 {
                continue;
            };
        }
        let mut eq = true;
        let scan_len = (i + 1).min(pattern[0].len() - i - 1);
        for row in pattern {
            if row[i + 1 - scan_len..i + 1]
                != row[i + 1..i + 1 + scan_len]
                    .iter()
                    .rev()
                    .copied()
                    .collect::<Vec<_>>()
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

    (ver_ref + 100 * hor_ref, ver_ref, hor_ref)
}
