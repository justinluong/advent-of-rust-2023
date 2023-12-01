use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn create_numeric_map() -> HashMap<&'static str, u8> {
    let mut map = HashMap::new();
    map.insert("zero", 0);
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map
}

fn extract_digits(line: &str, map: &HashMap<&'static str, u8>) -> u8 {
    let pattern = r"one|two|three|four|five|six|seven|eight|nine|\d";
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures_iter(line);
    caps.get(0).unwrap().as_str().to_digit();
}

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let map = create_numeric_map();

    for line in lines {
        let extracted_digits = extract_digits(line, map);
        println!("{}", extracted_digits);
        break
    }
}
