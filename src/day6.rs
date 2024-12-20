use ::std::error::Error;
use ::std::fs::read_to_string;

fn get_map(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let input = read_to_string(path)?;
    let mut map: Vec<String> = Vec::new();

    for line in input.lines() {
        map.push(line.to_string());
    }

    Ok(map)
}

fn find_init_conditions(map: Vec<String>) -> Option<(String, (usize, usize))> {
    let targets = ['^', '>', '<', 'v'];

    for (row_index, row_value) in map.iter().enumerate() {
        if let Some((index, matched)) = row_value
            .chars()
            .enumerate()
            .find(|&(_, c)| targets.contains(&c))
        {
            return Some((matched.to_string(), (row_index, index)));
        }
    }

    None
}

struct Guard {
    orientation: String,
    // row, column
    position: (usize, usize),
    map: Vec<String>,
}

impl Guard {
    fn walk(&mut self) -> Option<(usize, usize)> {
        let dimensions: (usize, usize) = (self.map.len(), self.map[0].chars().count());
        // Calculate move
        if (self.orientation.as_str() == "^" && self.position.0 == 0)
            || (self.orientation.as_str() == "<" && self.position.1 == 0)
        {
            return None;
        }

        // For some reason string literals like "^" are of type &str in Rust...
        let movement = match self.orientation.as_str() {
            "^" => (self.position.0 - 1, self.position.1),
            "v" => (self.position.0 + 1, self.position.1),
            ">" => (self.position.0, self.position.1 + 1),
            "<" => (self.position.0, self.position.1 - 1),
            &_ => todo!(),
        };

        // Check if move is obstructed
        // Handle possible out of bounds here...
        if movement.0 < dimensions.0 && movement.1 < dimensions.1 {
            let obstruction = self.map[movement.0].chars().nth(movement.1).unwrap();
            if obstruction == '#' {
                self.rotate();
                Some(self.position)
            } else {
                self.position = movement;
                Some(self.position)
            }
        } else {
            None
        }
    }

    fn rotate(&mut self) {
        self.orientation = match self.orientation.as_str() {
            "^" => ">".to_string(),
            ">" => "v".to_string(),
            "v" => "<".to_string(),
            "<" => "^".to_string(),
            // This is cool? What's this do...
            &_ => todo!(),
        }
    }
}

impl Iterator for Guard {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // Return valid progress otherwise None.
        self.walk()
    }
}

fn unique_positions(positions: Vec<(usize, usize)>) -> i32 {
    let mut unique_positions: Vec<(usize, usize)> = Vec::new();
    for position in positions {
        if !unique_positions.contains(&position) {
            unique_positions.push(position);
        }
    }
    unique_positions.len().try_into().unwrap()
}

fn next_position(position: (usize, usize), dimension: (usize, usize)) -> (usize, usize) {
    if position.0 < dimension.0 {
        (position.0, position.1 + 1)
    } else {
        (position.0 + 1, position.1)
    }
}

struct Map {
    map: Vec<String>,
    position: (usize, usize),
    dimension: (usize, usize),
}

impl Iterator for Map {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {}
}

pub fn part1() {
    let map = get_map("src/data/day6.txt".to_string()).expect("REASON");
    let initial_conditions = find_init_conditions(map.clone()).unwrap();
    let mut positions: Vec<(usize, usize)> = Vec::new();

    let g = Guard {
        orientation: initial_conditions.0,
        position: initial_conditions.1,
        map,
    };

    for guard_position in g {
        positions.push(guard_position);
    }
    positions.push(initial_conditions.1);

    println!("Part 1: {:?}", unique_positions(positions));
}

pub fn part2() {
    let map = get_map("src/data/day6.txt".to_string()).expect("REASON");
    let obstacles: Vec<i32> = Vec::new();

    let maps = Map {
        map: map,
        init: (0, 0),
    };

    for m in maps {
        let initial_conditions = find_init_conditions(m);
        let g = Guard {
            orientation: initial_conditions.0,
            position: initial_conditions.1,
            map,
        };

        for guard_position in g {
            // If we reach the same position and orientation, we know we are in a loop.
            // If this is discovered, break out of loop over positions into loop for next map.
            if guard_position.orientation =
                initial_conditions.0 && guard_position.position = initial_conditions.1
            {
                obstacles.push(1);
                break;
            }
        }
    }

    let result: i32 = obstacles.iter().sum();

    println!("Part 2: {:?}", result);
}
