use std::collections::{BTreeMap, BTreeSet};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_22_1.txt");

type LevelBlockMap = BTreeMap<usize, Vec<(usize, usize)>>;

fn main() {
    let mut blocks = INPUT_FILE
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            let (begin, end) = line.split_once('~').unwrap();
            let mut b = begin.split(',').map(|v| v.parse::<usize>().unwrap());
            let bx = b.next().unwrap();
            let by = b.next().unwrap();
            let bz = b.next().unwrap();
            let mut e = end.split(',').map(|v| v.parse::<usize>().unwrap());
            let ex = e.next().unwrap();
            let ey = e.next().unwrap();
            let ez = e.next().unwrap();

            (bx..=ex)
                .flat_map(move |x| std::iter::repeat(x).zip(by..=ey))
                .flat_map(move |v| std::iter::repeat(v).zip(bz..=ez))
                .map(move |((x, y), z)| (z, (x, y, idx)))
        })
        .fold(
            BTreeMap::<_, BTreeMap<_, Vec<_>>>::new(),
            |mut acc, (z, (x, y, block_id))| {
                acc.entry(z)
                    .or_default()
                    .entry(block_id)
                    .or_default()
                    .push((x, y));
                acc
            },
        );

    // Lower blocks to their final place
    for z in 2..=*blocks.last_key_value().unwrap().0 {
        if let Some(block_map) = blocks.get_mut(&z) {
            let block_map = std::mem::take(block_map);
            for (block_group, block_coords) in block_map {
                let mut new_z = z;
                for _ in 1..z {
                    if let Some(existing_blocks) = blocks.get(&(new_z - 1)) {
                        if existing_blocks
                            .values()
                            .flatten()
                            .any(|existing_block| block_coords.contains(existing_block))
                        {
                            break;
                        }
                    }
                    new_z -= 1;
                }
                blocks
                    .entry(new_z)
                    .or_default()
                    .entry(block_group)
                    .or_default()
                    .extend(block_coords);
            }
        }
    }

    // Ensure all layers are present
    for i in 1..*blocks.last_key_value().unwrap().0 {
        blocks.entry(i).or_default();
    }

    let total_chain = disintegrate_chain(&blocks, INPUT_FILE.lines().count());

    println!("Total blocks disintegrated in chains {total_chain}");
}

fn disintegrate_chain(blocks: &BTreeMap<usize, LevelBlockMap>, total_blocks: usize) -> usize {
    (0..total_blocks)
        .map(|block_id| {
            let mut blocks = blocks.clone();
            let mut min_eviscerated = *blocks.last_key_value().unwrap().0;
            for (z, block_map) in blocks.iter_mut() {
                if block_map.remove(&block_id).is_some() {
                    min_eviscerated = min_eviscerated.min(*z);
                };
            }

            // Lower blocks to their final place
            let mut blocks_lowered = BTreeSet::new();
            for z in min_eviscerated..=*blocks.last_key_value().unwrap().0 {
                if let Some(block_map) = blocks.get_mut(&z) {
                    let block_map = std::mem::take(block_map);
                    for (block_group, block_coords) in block_map {
                        let mut new_z = z;
                        for _ in 1..z {
                            if let Some(existing_blocks) = blocks.get(&(new_z - 1)) {
                                if existing_blocks
                                    .values()
                                    .flatten()
                                    .any(|existing_block| block_coords.contains(existing_block))
                                {
                                    break;
                                }
                            }
                            new_z -= 1;
                        }
                        blocks
                            .entry(new_z)
                            .or_default()
                            .entry(block_group)
                            .or_default()
                            .extend(block_coords);
                        if new_z != z {
                            blocks_lowered.insert(block_group);
                        }
                    }
                }
            }

            blocks_lowered.len()
        })
        .sum()
}
