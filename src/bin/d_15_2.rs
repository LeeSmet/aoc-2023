/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_15_1.txt");

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn main() {
    let mut boxes = vec![vec![]; 256];
    for line in INPUT_FILE.split(',').map(str::trim) {
        let (label, lens_length) = line.split_once(['-', '=']).unwrap();
        let box_id = hash(label);

        if lens_length.is_empty() {
            boxes[box_id].retain(|lens: &Lens| lens.label != label);
        } else {
            let focal_length = lens_length.parse().unwrap();
            if let Some(pos) = boxes[box_id].iter().position(|lens| lens.label == label) {
                boxes[box_id][pos] = Lens {
                    label: label.to_string(),
                    focal_length,
                };
            } else {
                boxes[box_id].push(Lens {
                    label: label.to_string(),
                    focal_length,
                })
            }
        }
    }

    let focus_power = boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.into_iter()
                .enumerate()
                .map(|(lens_idx, l)| l.focal_length * (1 + lens_idx) * (1 + box_idx))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Total focussing power is {focus_power}");
}

// Technically this returns a u8 since result is clamped to [0,255] but we use usize to avoid
// casting everywhere.
fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (((c as usize) + acc) * 17) % 256)
}
