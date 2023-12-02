use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

fn create_numeric_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("zero", "0");
    map.insert("one", "1");
    map.insert("two", "2");
    map.insert("three", "3");
    map.insert("four", "4");
    map.insert("five", "5");
    map.insert("six", "6");
    map.insert("seven", "7");
    map.insert("eight", "8");
    map.insert("nine", "9");
    map
}

fn digit_from_match(digit_match: &str, map: &HashMap<&'static str, &'static str>) -> String {
    match map.get(digit_match) {
        Some(&digit) => digit.to_string(),
        None => digit_match.to_string(),
    }
}

fn find_matches(line: &str) -> Vec<String> {
    let digit_words: HashSet<&str> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .cloned()
    .collect();
    let max_length = digit_words.iter().map(|word| word.len()).max().unwrap();

    let mut matches = Vec::new();

    for i in 0..line.len() {
        for j in i + 1..=line.len() {
            let current_str = &line[i..j];
            if current_str.len() == 1 && current_str.chars().next().unwrap().is_digit(10)
                || digit_words.contains(current_str)
            {
                matches.push(current_str.to_string());
                break;
            } else if current_str.len() > max_length {
                break;
            }
        }
    }

    matches
}

fn extract_digits(line: &str, map: &HashMap<&'static str, &'static str>) -> u32 {
    let matches: Vec<String> = find_matches(line);
    let first_digit = digit_from_match(&matches.first().unwrap(), map);
    let last_digit = digit_from_match(&matches.last().unwrap(), map);
    (first_digit + &last_digit).parse::<u32>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path_str = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "input.txt".to_string());
    let file_path = Path::new(&file_path_str);

    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let map = create_numeric_map();
    let calibration_values: Vec<u32> = lines
        .iter()
        .map(|line| extract_digits(line, &map))
        .collect();

    let sum: u32 = calibration_values.iter().map(|&x| x).sum();

    println!("Sum: {}", sum);
}
