use core::panic;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

fn calculate_matches(card: &str) -> u32 {
    let split_card: Vec<&str> = card.split(|c| c == '|' || c == ':').collect();
    let winning_nums: HashSet<u32> = split_card[1]
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    let your_nums: HashSet<u32> = split_card[2]
        .trim()
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    winning_nums.intersection(&your_nums).count() as u32
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let cards: Vec<&str> = content.lines().collect();

    let card_results: HashMap<usize, u32> = cards
        .iter()
        .enumerate()
        .map(|(index, card)| (index, calculate_matches(card)))
        .collect();

    let mut queue: Vec<usize> = (0..cards.len()).collect();

    let mut num_cards: usize = 0;

    while queue.len() > 0 {
        let card_number = queue.pop().unwrap();
        num_cards += 1;
        let card_result = card_results.get(&card_number).unwrap();
        for i in (card_number as u32 + 1)..=(card_number as u32 +card_result) {
            queue.push(i as usize);
        }
    }
    println!("{}", num_cards);
}
