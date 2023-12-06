use std::env;
use std::fs;

fn ways_to_win(race: &(u32, u32)) -> u32 {
    let  mut ways_to_win: u32 = 0;
    for hold_length in 0..=race.0 {
        let distance = hold_length * (race.0 - hold_length);
        if distance > race.1 {
            ways_to_win += 1
        }
    }
    ways_to_win
}

fn parse_races(input: &str) -> Vec<(u32, u32)> {
    let lines: Vec<&str> = input.lines().collect();
    let time_str: &str = lines[0];
    let times: Vec<u32> = time_str[time_str.find(":").unwrap() + 1..]
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let distance_str: &str = lines[1];
    let distances: Vec<u32> = distance_str[distance_str.find(":").unwrap() + 1..]
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    times.into_iter().zip(distances.into_iter()).collect()
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let races: Vec<(u32, u32)> = parse_races(&contents);
    let races_wtw: Vec<u32> = races.iter().map(|race| ways_to_win(&race)).collect();
    println!("{}", races_wtw.iter().product::<u32>());
}
