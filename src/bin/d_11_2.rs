/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_11_1.txt");

fn main() {
    let field: Vec<Vec<_>> = INPUT_FILE
        .lines()
        .map(|line| line.chars().collect())
        .collect();

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

    let mut galaxies = vec![];
    for (i, row) in field.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                let rid = i + rows_to_add.iter().filter(|&&r| r < i).count() * 999999;
                let cid = j + cols_to_add.iter().filter(|&&c| c < j).count() * 999999;
                galaxies.push((rid, cid));
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
