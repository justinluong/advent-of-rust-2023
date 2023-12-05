use std::env;
use std::fs;

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

fn parse_seeds(seeds_str: &str) -> Vec<u64> {
    let parts: Vec<&str> = seeds_str.split(" ").collect();
    parts[1..].iter().map(|num| num.parse().unwrap()).collect()
}

fn parse_range(range_str: &str) -> Range {
    let range_values: Vec<u64> = range_str
        .split(" ")
        .into_iter()
        .map(|num| num.parse().unwrap())
        .collect();
    Range {
        destination_start: range_values[0],
        source_start: range_values[1],
        range_length: range_values[2],
    }
}

fn parse_map(map_str: &str) -> Vec<Range> {
    // O(n+rlogr)
    // Where n is the length of the longest line and r is the number of ranges

    let range_lines: Vec<&str> = map_str[map_str.find(":").unwrap() + 1..]
        .trim()
        .lines()
        .collect();
    let mut parsed_ranges: Vec<Range> = range_lines.iter().map(|line| parse_range(line)).collect();
    parsed_ranges.sort_by_key(|range| range.source_start);
    parsed_ranges
}

fn do_mapping(source_val: u64, map: &Vec<Range>) -> u64 {
    for range in map {
        let source_end = match range.source_start.checked_add(range.range_length) {
            Some(end) => end,
            None => {
                eprintln!("Overflow occurred with source_start: {}, range_length: {}", range.source_start, range.range_length);
                continue; // Skip this iteration or handle the error as needed
            },
        };
        if range.source_start <= source_val && source_val < source_end
        {
            let diff = source_val - range.source_start;
            return range.destination_start + diff;
        }
    }
    source_val
}

fn calculate_location(seed: u64, maps: &Vec<Vec<Range>>) -> u64 {
    maps.iter().fold(seed, |acc, elem| do_mapping(acc, elem))
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let almanac_parts: Vec<&str> = contents.split("\n\n").collect();
    let seeds = parse_seeds(almanac_parts[0]);
    let maps: Vec<Vec<Range>> = almanac_parts[1..]
        .iter()
        .map(|map_str| parse_map(map_str))
        .collect();

    let locations: Vec<u64> = seeds.iter().map(|seed| calculate_location(*seed, &maps)).collect();
    println!("{}", locations.iter().min().unwrap());
}
