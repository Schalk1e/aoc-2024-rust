use ::std::error::Error;
use ::std::fs;

// Read and process input
fn process(path: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut result = Vec::new();
    let input: String = fs::read_to_string(path)?;
    // println!("{}", input);
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let ids = line.split("   ");
        let collection = ids.collect::<Vec<&str>>();

        list_a.push(collection[0].to_string());
        list_b.push(collection[1].to_string());
    }

    // Sort lists
    list_a.sort();
    list_b.sort();

    result.push(list_a);
    result.push(list_b);

    Ok(result)
}

// Part 1

// calculate the difference between each pair of numbers in input.txt
pub fn part1() -> Result<(), Box<dyn Error>> {
    let mut diffs: std::vec::Vec<i32> = Vec::new();

    let input = process("src/data/input.txt".to_string());

    // Zip together
    let binding = input.unwrap();
    let it = binding[0].iter().zip(binding[1].iter());

    for (a, b) in it {
        let d: i32 = a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap();
        diffs.push(d.abs());
        // diffs.push(d);
    }

    let result: i32 = diffs.iter().sum();

    println!("Part 1: {}", result);

    Ok(())
}

// Part 2

pub fn part2() -> Result<(), Box<dyn Error>> {
    let input = process("src/data/input.txt".to_string());
    let binding = input.unwrap();
    let mut frequencies = Vec::new();

    let lookup: &Vec<String> = &binding[0];

    for id in lookup {
        let frequency: i32 = binding[1]
            .iter()
            .filter(|&n| *n == *id) // What does dereferencing do?
            .count()
            .try_into()
            .unwrap();
        frequencies.push(id.parse::<i32>().unwrap() * frequency)
    }

    let result: i32 = frequencies.iter().sum();

    println!("Part 2: {}", result);

    Ok(())
}
