use ::std::error::Error;
use ::std::fs::read_to_string;
use itertools::Itertools;

fn input(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let input = read_to_string(path)?;
    let mut map: Vec<String> = Vec::new();

    for line in input.lines() {
        map.push(line.to_string());
    }

    Ok(map)
}

fn find_antennas_from_map(map: Vec<String>) -> Vec<(usize, usize)> {
    let mut antennas: Vec<(usize, usize)> = Vec::new();

    for (idx, value) in map.iter().enumerate() {
        for (str_idx, str_value) in value.chars().enumerate() {
            if str_value != '.' {
                antennas.push((idx, str_idx));
            }
        }
    }

    antennas
}

fn find_antinode(constant: i32, antenna_pair: Vec<(usize, usize)>) -> (i32, i32) {
    // Find antinodes with the middle point formula.
    let pair: Vec<(i32, i32)> = antenna_pair
        .iter()
        .map(|&(x, y)| (x as i32, y as i32))
        .collect();

    (
        constant * pair[0].0 - (constant - 1) * pair[1].0,
        constant * pair[0].1 - (constant - 1) * pair[1].1,
    )
}

fn find_antenna_pairs<T>(antennas: Vec<T>) -> Vec<Vec<T>>
// This means the generic type T must implement the clone trait.
where
    T: Clone,
{
    antennas.clone().into_iter().permutations(2).collect()
}

fn get_map_element(pair: (usize, usize), map: Vec<String>) -> char {
    let row = &map[pair.0];

    row.chars().nth(pair.1).unwrap()
}

fn filter_antenna_pairs(
    antenna_pairs: Vec<Vec<(usize, usize)>>,
    map: Vec<String>,
) -> Vec<Vec<(usize, usize)>> {
    let mut filtered_pairs: Vec<Vec<(usize, usize)>> = Vec::new();

    for pair in antenna_pairs {
        if get_map_element(pair[0], map.clone()) == get_map_element(pair[1], map.clone()) {
            filtered_pairs.push(pair)
        }
    }

    filtered_pairs
}

fn find_antinodes(max_constant: i32, antenna_pair: Vec<(usize, usize)>) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    for constant in 0..max_constant {
        let antinode = find_antinode(constant, antenna_pair.clone());
        antinodes.push(antinode);
    }

    antinodes
}

pub fn part1(no_print: bool) -> i64 {
    let map = input("src/data/day8.txt".to_string()).expect("REASON");
    let map_dimension: i32 = (map.len() - 1).try_into().unwrap();

    let antennas = find_antennas_from_map(map.clone());
    let antenna_pairs = find_antenna_pairs(antennas);
    let filtered_pairs = filter_antenna_pairs(antenna_pairs, map);

    let mut antinodes_in_area: Vec<(i32, i32)> = Vec::new();
    for antenna_pair in filtered_pairs {
        let antinode = find_antinode(2, antenna_pair);
        if antinode.0 >= 0
            && antinode.0 <= map_dimension
            && antinode.1 >= 0
            && antinode.1 <= map_dimension
            && !antinodes_in_area.contains(&antinode)
        {
            antinodes_in_area.push(antinode);
        }
    }

    let counts: i64 = antinodes_in_area.len().try_into().unwrap();

    if !no_print {
        println!("Part 1: {:?}", counts);
    }

    counts
}

pub fn part2(no_print: bool) -> i64 {
    let map = input("src/data/day8.txt".to_string()).expect("REASON");
    let map_dimension: i32 = (map.len() - 1).try_into().unwrap();

    let antennas = find_antennas_from_map(map.clone());
    let antenna_pairs = find_antenna_pairs(antennas);
    let filtered_pairs = filter_antenna_pairs(antenna_pairs, map);

    let mut antinodes_in_area: Vec<(i32, i32)> = Vec::new();
    for antenna_pair in filtered_pairs {
        let antinodes = find_antinodes(map_dimension, antenna_pair);
        for antinode in antinodes {
            if antinode.0 >= 0
                && antinode.0 <= map_dimension
                && antinode.1 >= 0
                && antinode.1 <= map_dimension
                && !antinodes_in_area.contains(&antinode)
            {
                antinodes_in_area.push(antinode);
            }
        }
    }

    let counts: i64 = antinodes_in_area.len().try_into().unwrap();

    if !no_print {
        println!("Part 2: {:?}", counts);
    }

    counts
}
