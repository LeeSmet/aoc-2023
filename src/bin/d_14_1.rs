/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_14_1.txt");

fn main() {
    let mut field = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 1..field.len() {
        for j in (0..i).rev() {
            for k in 0..field[i].len() {
                if field[j][k] == '.' && field[j + 1][k] == 'O' {
                    field[j][k] = 'O';
                    field[j + 1][k] = '.';
                }
            }
        }
    }

    let load = field
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (field.len() - i))
        .sum::<usize>();

    println!("Total load is {load}");
}
