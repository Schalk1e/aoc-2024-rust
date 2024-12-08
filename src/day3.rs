use regex::Regex;
use std::fs;

fn extract_mul_patterns(input: String) -> Vec<String> {
    let mut results = Vec::new();
    // match pattern mul(digit,digit)
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    for mat in re.find_iter(&input) {
        results.push(mat.as_str().to_string());
    }

    results
}

fn multiply_from_mul_pattern(pattern: String) -> i32 {
    let mut digits: Vec<String> = Vec::new();
    // match pattern digit
    let re = Regex::new(r"\d+").unwrap();

    for mat in re.find_iter(&pattern) {
        digits.push(mat.as_str().to_string());
    }

    let result: i32 = digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();

    result
}

fn extract_do_mul_patterns(input: String) -> Vec<String> {
    let mut results = Vec::new();
    // match all text between the strings do and don't.
    // NOTE: Because I hate regex I simplified this by adding a do to the
    // start and don't to the end of the data file.
    let re = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();

    for mat in re.find_iter(&input) {
        // println!("{}", mat.as_str());
        results.push(mat.as_str().to_string());
    }

    results
}

pub fn part1() {
    let input = fs::read_to_string("src/data/day3.txt");
    let patterns = extract_mul_patterns(input.expect("REASON"));

    let mut sums: Vec<i32> = Vec::new();

    for pattern in patterns {
        sums.push(multiply_from_mul_pattern(pattern));
    }

    let result: i32 = sums.iter().sum();

    println!("Part 1: {}", result);
}

pub fn part2() {
    let input = fs::read_to_string("src/data/day3.txt");
    let patterns: Vec<String> =
        extract_mul_patterns(extract_do_mul_patterns(input.expect("REASON")).join(""));

    let mut sums: Vec<i32> = Vec::new();

    for pattern in patterns {
        sums.push(multiply_from_mul_pattern(pattern));
    }

    let result: i32 = sums.iter().sum();

    println!("Part 1: {}", result);
}
