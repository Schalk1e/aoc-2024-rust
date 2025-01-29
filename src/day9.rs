use std::error::Error;
use std::fs::read_to_string;

fn input(path: String) -> Result<String, Box<dyn Error>> {
    let input = read_to_string(path)?;

    Ok(input)
}

pub fn part1() {
    let input = input("src/data/day9.txt".to_string()).expect("REASON");

    println!("{:?}", input);
}
