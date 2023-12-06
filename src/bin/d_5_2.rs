use std::ops::Range;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_5_1.txt");

fn main() {
    let mut input_lines = INPUT_FILE.lines();
    let mut seeds = input_lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|d| d.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    seeds.sort_by(|f, s| f.start.cmp(&s.start));

    input_lines.next();
    input_lines.next();
    let mut seed_to_soil: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    seed_to_soil.sort_by(|f, s| f.0.start.cmp(&s.0.start));

    input_lines.next();
    let mut soil_to_fertilizer: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    soil_to_fertilizer.sort_by(|f, s| f.0.start.cmp(&s.0.start));

    input_lines.next();
    let mut fertilizer_to_water: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    fertilizer_to_water.sort_by(|f, s| f.0.start.cmp(&s.0.start));

    input_lines.next();
    let mut water_to_light: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    water_to_light.sort_by(|f, s| f.0.start.cmp(&s.0.start));

    input_lines.next();
    let mut light_to_temperature: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    light_to_temperature.sort_by(|f, s| f.0.start.cmp(&s.0.start));

    input_lines.next();
    let mut temperature_to_humidity: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    temperature_to_humidity.sort_by(|f, s| f.0.start.cmp(&s.0.start));

    input_lines.next();
    let mut humidity_to_location: Vec<(Range<u64>, Range<u64>)> = input_lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(line_to_vals)
        .collect();
    humidity_to_location.sort_by(|f, s| f.0.start.cmp(&s.0.start));

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
        .flat_map(|range| {
            let mut ranges = vec![range.clone()];
            for mapping in &mappings {
                let mut new_ranges = vec![];
                for range in ranges {
                    new_ranges.extend_from_slice(&find_split_ranges(range, mapping));
                }
                ranges = new_ranges;
            }
            ranges
        })
        .map(|r| r.start)
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

/// find all mapped ranges, mapping range must be sorted.
fn find_split_ranges(r: Range<u64>, mapping: &[(Range<u64>, Range<u64>)]) -> Vec<Range<u64>> {
    let mut ranges = vec![];
    for (src_range, dst_range) in mapping {
        // If the original range is entirely before this range we are done since all next ranges
        // (if any) come after that one.
        if r.end < src_range.start {
            break;
        }
        // If the src range is entirely before our original range, skip to the next range
        if src_range.end <= r.start {
            continue;
        }
        // 4 cases now:
        //   - r starts before and ends in src
        //   - r starts in src and ends after src
        //   - r starts in src and ends in src
        //   - r starts before and ends after src
        //
        // We first check if r starts before src. If it does, we map the part before src and
        // recurse with the remainder, which will then be case 2 or 3.
        // If r starts in src, we check how many items are in source. We map those, and if r has a
        // remainder (past src), we recurse with that.
        if r.start < src_range.start {
            // Identity map this part
            ranges.push(r.start..src_range.start);
            // Mapped part
            ranges.extend_from_slice(&find_split_ranges(src_range.start..r.end, mapping));
            continue;
        }
        let end = r.end.min(src_range.end);
        let offset = r.start - src_range.start;
        let count = src_range.end.min(r.end) - r.start;
        // Mapped part
        ranges.push(dst_range.start + offset..dst_range.start + offset + count);
        if end >= src_range.end {
            ranges.extend_from_slice(&find_split_ranges(src_range.end..r.end, mapping));
        }
        // We mapped for sure here
        break;
    }
    // Our original range is either in front of or past all mappings, identity map it
    if ranges.is_empty() {
        ranges.push(r);
    }
    ranges
}
