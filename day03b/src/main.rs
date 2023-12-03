use std::collections::HashSet;
use std::env;
use std::fs;

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data[0].len()
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.rows() || col >= self.cols() {
            None
        } else {
            Some(self.data[row][col])
        }
    }
}

fn identify_number(grid: &Grid, row: usize, col: usize) -> (u32, HashSet<(usize, usize)>) {
    let mut start: usize = col;
    let mut end: usize = col;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut i = col as isize;
    while i >= 0 {
        if grid.get(row, i as usize).unwrap().is_digit(10) {
            start = i as usize;
            i -= 1;
        } else {
            break;
        }
    }

    let mut j = col;
    while j < grid.cols() {
        if grid.get(row, j).unwrap().is_digit(10) {
            end = j;
            j += 1;
        } else {
            break;
        }
    }

    let mut digits: Vec<char> = Vec::new();
    for k in start..=end {
        visited.insert((row, k));
        digits.push(grid.get(row, k).unwrap());
    }

    let number: String = digits.iter().collect();
    (number.parse::<u32>().unwrap(), visited)
}

fn calculate_gear_ratio(grid: &Grid, row: usize, col: usize) -> Option<u32> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut neighbouring_numbers: Vec<u32> = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            let cur_row = (row as isize + i) as usize;
            let cur_col = (col as isize + j) as usize;
            if !visited.contains(&(cur_row, cur_col))
                && grid.get(cur_row, cur_col).is_some()
                && grid.get(cur_row, cur_col).unwrap().is_digit(10)
            {
                let (number, cur_visited) = identify_number(grid, cur_row, cur_col);
                neighbouring_numbers.push(number);
                visited.extend(cur_visited);
            }
        }
    }

    if neighbouring_numbers.len() == 2 {
        Some(neighbouring_numbers.iter().product())
    } else {
        None
    }
}

fn find_gear_ratios(grid: &Grid) -> Vec<u32> {
    let mut gear_ratios: Vec<u32> = Vec::new();

    let mut row = 0;
    while row < grid.rows() {
        let mut col = 0;
        while col < grid.cols() {
            if let Some('*') = grid.get(row, col) {
                if let Some(value) = calculate_gear_ratio(grid, row, col) {
                    gear_ratios.push(value);
                }
            }
            col += 1;
        }
        row += 1
    }

    gear_ratios
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    let vec_grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let grid = Grid { data: vec_grid };

    let gear_ratios: Vec<u32> = find_gear_ratios(&grid);

    println!("{}", gear_ratios.iter().sum::<u32>());
}
