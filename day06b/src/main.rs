use std::env;
use std::fs;

/*
Efficient way to do this
find the first time to beat it. Multiply by 2
*/

fn wins(hold_length: u64, time: u64, record: u64) -> bool {
    let distance = hold_length * (time - hold_length);
    distance > record
}

fn ways_to_win(race: &(u64, u64)) -> u64 {
    /*
    0, 1, 1, 1, 1, 0
    0, 1, 1, 1, 1, 1, 0 -> 35765 is exactly in the middle
    */
    let time = race.0;
    let record = race.1;
    let mut low: u64 = 0;
    let mut high: u64 = race.0 / 2;
    let mut first_win: u64 = 1;
    while low < high {
        if low == high - 1 || low == high{
            first_win = high;
            break
        }

        let mut mid: u64 = (low + high) / 2;
        let mut low_result = wins(low, time, record);
        let mut mid_result = wins(mid, time, record);
        let mut high_result = wins(high, time, record);
        if mid_result == high_result {
            high = mid;
        } else {
            low = mid;
        }
    }

    let num_losses: u64 = first_win * 2;
    time + 1 - num_losses

}

fn parse_races(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let time_str: &str = lines[0];
    let time: u64 = time_str[time_str.find(":").unwrap() + 1..]
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let distance_str: &str = lines[1];
    let distance: u64 = distance_str[distance_str.find(":").unwrap() + 1..]
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    (time, distance)
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let race: (u64, u64) = parse_races(&contents);
    println!("{}", ways_to_win(&race));
}
