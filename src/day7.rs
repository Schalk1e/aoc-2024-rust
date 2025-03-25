use ::std::error::Error;
use ::std::fs::read_to_string;

fn input(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let input = read_to_string(path)?;
    let mut equations: Vec<String> = Vec::new();

    for equation in input.lines() {
        equations.push(equation.to_string());
    }

    Ok(equations)
}

fn parse_equation(equation: String) -> (i64, Vec<i64>) {
    // Find answer of equation
    let split_equation = equation.split(":");
    let split_vector = split_equation.collect::<Vec<&str>>();
    // i32 possibly too small, try i64...
    let result = split_vector[0].parse::<i64>().unwrap();

    // Construct terms on right
    let binding = split_vector[1].to_string();
    let split_terms = binding.split(" ");
    let mut terms = split_terms
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string())
        // So we can use remove.
        .collect::<Vec<String>>();

    // .remove returns the removed element not the vector!
    // need to remove before returning
    if !terms.is_empty() {
        terms.remove(0);
    }

    let iterms = terms
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (result, iterms)
}

fn binary_number_generator(bits: usize) -> Vec<String> {
    // Initialise permutations vector.
    let mut permutations = Vec::new();

    // Iterate over integers from 0 - 2 to the power 'bits'
    // 1 << bits is a bitwise left shift operator, whatever this means :D
    for i in 0..(1 << bits) {
        permutations.push(format!("{:0width$b}", i, width = bits));
    }

    permutations
}

fn calculate_binary_result(binary_string: String, terms: Vec<i64>) -> i64 {
    let mut init: usize = 0;
    let mut result: i64 = terms[0];

    for s in binary_string.chars() {
        if s == '0' {
            // Equivalent to result = result + terms[init + 1]
            result += terms[init + 1];
        } else {
            result *= terms[init + 1];
        }
        init += 1;
    }

    result
}

#[allow(dead_code)]
fn base_n_string(mut number: usize, base: usize) -> String {
    assert!(base >= 2, "Base must be at least 2");

    let mut result = String::new();

    while number > 0 {
        // Get the remainder (current digit in the base)
        let digit = number % base;
        // Convert digit to a character (supports bases up to 36: 0-9 and A-Z)
        result.push(std::char::from_digit(digit as u32, base as u32).unwrap());
        // Move to the next most significant digit
        number /= base;
    }

    // If the number is 0, return "0"
    if result.is_empty() {
        result.push('0');
    }

    // The result is constructed in reverse, so reverse it
    result.chars().rev().collect()
}

#[allow(dead_code)]
fn ternary_number_generator(positions: usize) -> Vec<String> {
    let mut permutations = Vec::new();

    for i in 0..3_usize.pow(positions as u32) {
        permutations.push(format!(
            "{:0>width$}",
            base_n_string(i, 3),
            width = positions
        ));
    }

    permutations
}

#[allow(dead_code)]
fn calculate_ternary_result(ternary_string: String, terms: Vec<i64>) -> i64 {
    let mut init: usize = 0;
    let mut result: i64 = terms[0];

    for s in ternary_string.chars() {
        if s == '0' {
            result += terms[init + 1];
        } else if s == '1' {
            result *= terms[init + 1];
        } else if s == '2' {
            let concatenated = format!("{}{}", result, terms[init + 1]);
            let concatenated_integer = concatenated.parse::<i64>().expect("REASON");
            result = concatenated_integer
        }

        init += 1;
    }

    result
}

pub fn part1() {
    let mut sums: Vec<i64> = Vec::new();

    let equations = input("src/data/day7.txt".to_string());

    match equations {
        Ok(equations) => {
            for equation in equations {
                let parsed_equation = parse_equation(equation);
                let permutations = binary_number_generator(parsed_equation.1.len() - 1);
                for permutation in &permutations {
                    let result =
                        calculate_binary_result(permutation.to_string(), parsed_equation.1.clone());
                    // println!("{:?}", result);
                    if result == parsed_equation.0 {
                        sums.push(result);
                        break;
                    }
                }
                // println!("{:?}", parsed_equation);
                // println!("{:?}", permutations);
            }
        }
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }

    let result: i64 = sums.iter().sum();

    println!("Part 1: {:?}", result);
}

pub fn part2() {
    let mut sums: Vec<i64> = Vec::new();

    let equations = input("src/data/day7.txt".to_string());

    match equations {
        Ok(equations) => {
            for equation in equations {
                let parsed_equation = parse_equation(equation);
                let permutations = ternary_number_generator(parsed_equation.1.len() - 1);
                for permutation in &permutations {
                    let result = calculate_ternary_result(
                        permutation.to_string(),
                        parsed_equation.1.clone(),
                    );
                    if result == parsed_equation.0 {
                        sums.push(result);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }

    let result: i64 = sums.iter().sum();

    println!("Part 2: {:?}", result);
}
