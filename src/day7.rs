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

fn parse_equation(equation: String) -> (i32, Vec<i32>) {
    // Find answer of equation
    let split_equation = equation.split(":");
    let split_vector = split_equation.collect::<Vec<&str>>();
    let result = split_vector[0].parse::<i32>().unwrap();

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
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

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

fn calculate_result(binary_string: String, terms: Vec<i32>) -> i32 {
    let mut init: usize = 0;
    let mut result: i32 = terms[0];

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

pub fn part1() {
    let mut sums: Vec<i32> = Vec::new();

    let equations = input("src/data/day7.txt".to_string());

    match equations {
        Ok(equations) => {
            for equation in equations {
                let parsed_equation = parse_equation(equation);
                let permutations = binary_number_generator(parsed_equation.1.len() - 1);
                for permutation in &permutations {
                    let result =
                        calculate_result(permutation.to_string(), parsed_equation.1.clone());
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

    let result: i32 = sums.iter().sum();

    println!("Part 1: {:?}", result);
}
