use std::env;
use std::fs;
use std::panic;

/*
Plan of attack
function to iterate over the entire grid
function to search around a number
function to move to the end of the number once searched
I'll clean later
*/

fn identify_number(line: &Vec<char>, j: usize) -> (u32, usize) {
    // Returns the number along with the index of the last digit
    let mut digits: Vec<char> = Vec::new();
    let mut k = j;
    while k < line.len() && line[k].is_digit(10) {
        digits.push(line[k]);
        k += 1;
    } // Finds the index of the last char in line or last char in number
    let number: String = digits.into_iter().collect();
    (number.parse::<u32>().unwrap(), k)
}

fn check_digit(grid: &Vec<Vec<char>>, i: usize, k: usize) -> bool {
    let mut coords: Vec<(usize, usize)> = Vec::new();

    for x in -1..=1 {
        for y in -1..=1 {
            coords.push((
                (i as isize + x) as usize,
                (k as isize + y) as usize
            ));
        }
    }

    for coord in coords {
        let result = panic::catch_unwind(|| {
            // This block is protected from panics
            grid[coord.0][coord.1]
        });

        let item = match result {
            Ok(value) => value,
            Err(_) => continue, // Skip to the next iteration if a panic occurred
        };

        if !item.is_digit(10) && item != '.' {
            return true
        }
    }
    false
}

fn is_part(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut k = j;

    while k < grid[0].len() && grid[i][k].is_digit(10) {
        if check_digit(grid, i, k) {
            return true
        }
        k += 1;
    }
    false
}

fn find_parts(grid: &Vec<Vec<char>>) -> Vec<u32> {
    let mut parts: Vec<u32> = Vec::new();

    let mut i = 0;
    while i < grid.len() {
        let mut j = 0;
        while j < grid[0].len() {
            let current_item = grid[i][j];
            if current_item.is_digit(10) {
                let number_details = identify_number(&grid[i], j);
                if is_part(grid, i, j) {
                    parts.push(number_details.0);
                }
                j = number_details.1 + 1;
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    parts
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let parts = find_parts(&grid);
    println!("{}", parts.iter().sum::<u32>());
}
