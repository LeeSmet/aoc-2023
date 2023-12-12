/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_12_1.txt");

fn main() {
    let possibilities = INPUT_FILE
        .lines()
        .map(|line| {
            let (input, counts) = line.split_once(' ').unwrap();
            let mut input = input.chars().collect::<Vec<_>>();
            let counts = counts
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            valid_lines(&mut input, &counts)
        })
        .sum::<usize>();

    println!("Total valid row arrangements {possibilities}");
}

fn valid_lines(line: &mut [char], expected_groups: &[usize]) -> usize {
    if let Some(pos) = line.iter().position(|&c| c == '?') {
        line[pos] = '.';
        let first = valid_lines(line, expected_groups);
        line[pos] = '#';
        let second = valid_lines(line, expected_groups);
        // restore to original value
        line[pos] = '?';
        first + second
    } else if line_valid(line, expected_groups) {
        1
    } else {
        0
    }
}

fn line_valid(line: &[char], expected_groups: &[usize]) -> bool {
    let mut groups = vec![];
    let mut last_char = '.';
    let mut cur_group_size = 0;
    for &c in line {
        if c == '#' {
            cur_group_size += 1;
        }
        if c == '.' && last_char == '#' {
            groups.push(cur_group_size);
            cur_group_size = 0;
        }
        last_char = c;
    }
    // If we end with a damaged part
    if cur_group_size > 0 {
        groups.push(cur_group_size);
    }

    expected_groups == groups
}
