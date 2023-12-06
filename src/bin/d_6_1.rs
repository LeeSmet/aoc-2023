/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_6_1.txt");

fn main() {
    let mut input_lines = INPUT_FILE.lines();

    let times = input_lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(str::trim)
        .map(str::parse::<u64>)
        .map(Result::unwrap);
    let distances = input_lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(str::trim)
        .map(str::parse::<u64>)
        .map(Result::unwrap);

    let opts = times
        .zip(distances)
        .map(|(time, distance)| {
            // Calculate the amount of possible ways the distance can be achieved. Since distance is
            // linear in time and speed, and speed can be written as a function of time, the distance
            // a ship travels is t * (T - t) = D ==> -t^2 + Tt = D. Solving this for t gives
            //
            //  (T - sqrt(T^2 - 4D)) / 2 and (T + sqrt(T^2 - 4D))

            // First equality, the first integer valid must be bigger or equal to this
            let inf1 =
                ((time as f64 - ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.).ceil() as u64;
            // First equality, the last integer valid must be smaller or equal to this
            let inf2 =
                ((time as f64 + ((time.pow(2) - 4 * distance) as f64).sqrt()) / 2.).floor() as u64;

            inf2 + 1 - inf1
        })
        .reduce(|acc, c| acc * c)
        .unwrap();

    println!("There are {opts} possible ways to win the race");
}
