use std::fs;
use std::collections::HashMap;
use std::path::Path;
use regex::Regex;

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

fn extract_digits(line: &str, map: &HashMap<&'static str, &'static str>) -> u32 {
    let pattern = r"one|two|three|four|five|six|seven|eight|nine|\d";
    let re = Regex::new(pattern).unwrap();
    let matches: Vec<_> = re.captures_iter(line).collect();
    let first_match: &str = matches.first().unwrap().get(0).unwrap().as_str();

    let mut first_digit = "";
    let mut last_digit = "";

    if first_match.len() != 1 {
        first_digit = map.get(first_match).unwrap();
    } else {
        first_digit = first_match;
    }
    let last_match: &str = matches.last().unwrap().get(0).unwrap().as_str();
    if last_match.len() != 1 {
        last_digit = map.get(last_match).unwrap();
    } else {
        last_digit = last_match;
    }

    println!("{}", first_digit);
    println!("{}", last_digit);
    1
}

fn main() {
    let file_path = Path::new("./input.txt");
    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let map = create_numeric_map();

    for line in lines {
        let extracted_digits = extract_digits(line, &map);
        println!("{}", extracted_digits);
        break
    }
}
