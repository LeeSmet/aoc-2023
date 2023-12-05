use std::{collections::HashMap, ops::Range};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_5_1.txt");

fn main() {
    let mut input_lines = INPUT_FILE.lines();
    let seeds: Vec<u64> = input_lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|d| d.parse().unwrap())
        .collect();

    input_lines.next();
    input_lines.next();
    let seed_to_soil: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    input_lines.next();
    let soil_to_fertilizer: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    input_lines.next();
    let fertilizer_to_water: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    input_lines.next();
    let water_to_light: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    input_lines.next();
    let light_to_temperature: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    input_lines.next();
    let temperature_to_humidity: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    input_lines.next();
    let humidity_to_location: HashMap<Range<u64>, Range<u64>> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();

    let mappings = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    let closest = seeds
        .iter()
        .map(|&v| {
            let mut v = v;
            for mapping in &mappings {
                for (src_range, dst_range) in mapping {
                    if src_range.contains(&v) {
                        let offset = v - src_range.start;
                        v = dst_range.start + offset;
                        break;
                    }
                }
            }
            v
        })
        .min()
        .unwrap();

    println!("Closest location to plant is {closest}");
}

fn line_to_vals(line: &str) -> (Range<u64>, Range<u64>) {
    let mut parts = line.split(' ');
    let dst: u64 = parts.next().unwrap().parse().unwrap();
    let src: u64 = parts.next().unwrap().parse().unwrap();
    let amount: u64 = parts.next().unwrap().parse().unwrap();

    (src..src + amount, dst..dst + amount)
}
