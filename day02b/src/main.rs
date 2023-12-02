use std::cmp::max;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct ColourSet {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl ColourSet {
    fn new() -> ColourSet {
        ColourSet { red: None, green: None, blue: None}
    }

    fn power(&self) -> u32 {
        self.red.unwrap_or(1) * self.green.unwrap_or(1) * self.blue.unwrap_or(1)
    }
}

fn parse_set(set_string: &str) -> ColourSet {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let matches = re.captures_iter(set_string);
    let mut colour_set = ColourSet::new();
    for m in matches {
        let count = m.get(1).unwrap().as_str().parse().ok();
        match m.get(2).unwrap().as_str() {
            "red" => colour_set.red = count,
            "green" => colour_set.green = count,
            "blue" => colour_set.blue = count,
            _ => {},
        }
    }
    colour_set
}

fn parse_game(game: &str) -> Vec<ColourSet> {
    let set_strings: Vec<&str> = game.split(";").collect();
    set_strings.iter().map(|&set_string| parse_set(set_string)).collect()
}

fn find_minimum_set(colour_sets: &Vec<ColourSet>) -> ColourSet {
    let mut max_red: Option<u32> = None;
    let mut max_green: Option<u32> = None;
    let mut max_blue: Option<u32> = None;

    for colour_set in colour_sets {
        max_red = max(max_red, colour_set.red);
        max_green = max(max_green, colour_set.green);
        max_blue = max(max_blue, colour_set.blue);
    }
    ColourSet { red: max_red, green: max_green, blue: max_blue }
}

fn calculate_minimum_set_power(game: &str) -> u32 {
    let colour_sets = parse_game(&game);
    let minimum_set = find_minimum_set(&colour_sets);
    minimum_set.power()
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();

    let result: u32 = lines.iter().map(|&game| calculate_minimum_set_power(game)).sum();
    println!("Result: {}", result);
}
