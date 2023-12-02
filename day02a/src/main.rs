use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

/*
Logic
-> divide lines into games
-> analyse game
-> split game into sets
-> check if set is valid
*/

fn parse_game(game_str: &str) -> Vec<HashMap<String, u8>> {
    let game_set_split: Vec<&str> = game_str.split(":").collect();
    let set_strings: Vec<&str> = game_set_split[1].split(";").collect();
    let re = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut sets: Vec<HashMap<String, u8>> = Vec::new();

    for set in set_strings.iter() {
        let mut set_map: HashMap<String, u8> = HashMap::new();
        let matches = re.captures_iter(set);

        for m in matches {
            let colour = m[2].to_string();
            let count = u8::from_str(&m[1]).unwrap();
            set_map.insert(colour, count);
        }

        sets.push(set_map);
    }

    sets
}

fn check_game_validity(game: &str, cube_limit_map: &HashMap<String, u8>) -> bool {
    let sets = parse_game(game);
    for set in sets {
        for (colour, count) in set.iter() {
            let colour_limit: Option<&u8> = cube_limit_map.get(colour);
            if colour_limit.is_none() || count > colour_limit.unwrap() {
                return false;
            }
        }
    }
    true
}

fn create_cube_limit_map() -> HashMap<String, u8> {
    let mut map: HashMap<String, u8> = HashMap::new();
    map.insert("red".to_string(), 12);
    map.insert("green".to_string(), 13);
    map.insert("blue".to_string(), 14);
    map
}

fn main() {
    let file_path: String = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let cube_limit_map = create_cube_limit_map();
    let re_game_number = Regex::new(r"\d+").unwrap();

    let mut valid_games: Vec<u32> = Vec::new();

    for line in lines {
        let game_number = re_game_number
            .captures(line)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        if check_game_validity(&line, &cube_limit_map) {
            valid_games.push(game_number);
        }
    }

    println!("Sum: {}", valid_games.iter().sum::<u32>());
}
