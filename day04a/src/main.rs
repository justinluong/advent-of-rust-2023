use std::env;
use std::fs;
use std::collections::HashSet;

fn calculate_points(card: &str) -> u32 {
    let split_card: Vec<&str> = card.split(|c| c == '|' || c == ':').collect();
    let winning_nums: HashSet<u32> = split_card[1].trim().split_whitespace().map(|c| c.parse().unwrap()).collect();
    let your_nums: HashSet<u32> = split_card[2].trim().split_whitespace().map(|c| c.parse().unwrap()).collect();
    let num_matches = winning_nums.intersection(&your_nums).count() as u32;

    if num_matches == 0 {
        return 0
    } else {
        2u32.pow(num_matches-1)
    }
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let cards: Vec<&str> = content.lines().collect();

    let total_points: u32 = cards.iter().map(|card| calculate_points(card)).sum();

    println!("{}", total_points);
}
