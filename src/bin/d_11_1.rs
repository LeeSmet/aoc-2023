/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_11_1.txt");

fn main() {
    let mut field: Vec<Vec<_>> = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = field.len();
    let cols = field[0].len();

    let mut rows_to_add = vec![];
    let mut cols_to_add = vec![];

    // Where do we add rows
    for (i, row) in field.iter().enumerate() {
        if row.iter().all(|c| c == &'.') {
            rows_to_add.push(i);
        }
    }

    for j in 0..cols {
        if field.iter().map(|r| r[j]).all(|c| c == '.') {
            cols_to_add.push(j);
        }
    }

    for (idx, col) in cols_to_add.into_iter().enumerate() {
        for row in field.iter_mut().take(rows) {
            row.insert(idx + col, '.');
        }
    }

    for (idx, row) in rows_to_add.into_iter().enumerate() {
        field.insert(idx + row, field[idx + row].clone());
    }

    let mut galaxies = vec![];
    for (i, row) in field.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut distances = vec![];
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let distance =
                galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
            distances.push((i, j, distance))
        }
    }

    println!(
        "Distance between galaxies is {}",
        distances.iter().map(|x| x.2).sum::<usize>()
    );
}
