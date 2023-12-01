use std::fs;

fn get_calibration_value(line: &str) -> u32 {
    let digit_chars: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    if digit_chars.is_empty() {
        panic!("No digits found in the input line");
    } else {
        let first_digit = digit_chars.first().unwrap();
        let last_digit = digit_chars.last().unwrap();
        let digits = first_digit.to_string() + &last_digit.to_string();
        digits.parse::<u32>().unwrap()
    }
}

fn main() {
    let file_path  = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have read lines");

    let lines: Vec<&str> = contents.lines().collect();

    let calibration_values: Vec<u32> = lines.iter().map(|line| get_calibration_value(line)).collect();

    let sum: u32 = calibration_values.iter().map(|&x| x).sum();

    println!("Sum: {}", sum);
}
