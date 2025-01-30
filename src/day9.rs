use std::error::Error;
use std::fs::read_to_string;

fn input(path: String) -> Result<String, Box<dyn Error>> {
    let input = read_to_string(path)?;
    // Trim the newline
    let input_trimmed = input.trim();

    Ok(input_trimmed.to_string())
}

fn file_blocks(disk_map: String) -> String {
    let mut id = 0;
    let mut file_blocks = String::from("");
    for (idx, value) in disk_map.chars().enumerate() {
        // Check if we are on a even or odd number
        if idx % 2 == 0 {
            // If even, we want a block of length n
            if let Some(num) = value.to_digit(10) {
                let n: usize = num as usize;
                let block = id.to_string().repeat(n);
                file_blocks.push_str(&block);
                id += 1
            }
        } else if let Some(num) = value.to_digit(10) {
            let n: usize = num as usize;
            let block = ".".to_string().repeat(n);
            file_blocks.push_str(&block);
        }
    }

    file_blocks
}

fn move_blocks(file_blocks: String) -> String {
    let mut moved_blocks = String::from("");

    // Convert file blocks string to vector of chars
    let mut chars: Vec<char> = file_blocks.chars().collect();

    // Cycle through it and pop elements off the front and back as you go
    while !chars.is_empty() {
        if chars[0] != '.' {
            moved_blocks.push(chars[0]);
            chars.remove(0);
        } else if chars[0] == '.' {
            let s = chars.pop();
            if s != Some('.') {
                moved_blocks.push(s.expect("REASON"));
                chars.remove(0);
            }
        }
    }

    moved_blocks
}

fn checksum(moved_blocks: String) -> usize {
    let mut elements: Vec<usize> = Vec::new();
    for (idx, value) in moved_blocks.chars().enumerate() {
        let product = idx * value.to_digit(10).expect("REASON") as usize;
        elements.push(product);
    }

    elements.iter().sum()
}

pub fn part1() {
    let input = input("src/data/day9.txt".to_string()).expect("REASON");

    println!("{:?}", input);

    let blocks = file_blocks(input);

    let moved_blocks = move_blocks(blocks);

    let csum = checksum(moved_blocks);

    println!("Part 1: {:?}", csum);
}
