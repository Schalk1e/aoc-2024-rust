use ::std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct CrosswordElement {
    // (row, column)
    position: (usize, usize),
    // (row, column)
    dimension: (usize, usize),
    // matrix
    matrix: Vec<Vec<char>>,
}

impl CrosswordElement {
    // Check whether forward elements have an XMAS entry.
    fn has_horisontal_match(&self) -> i32 {
        let space_to_right: usize = self.dimension.1 - self.position.1;
        if space_to_right >= 4 {
            let slice = (
                self.matrix[self.position.0][self.position.1],
                self.matrix[self.position.0][self.position.1 + 1],
                self.matrix[self.position.0][self.position.1 + 2],
                self.matrix[self.position.0][self.position.1 + 3],
            );
            match slice {
                ('X', 'M', 'A', 'S') => 1,
                ('S', 'A', 'M', 'X') => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
    fn has_vertical_match(&self) -> i32 {
        let space_below: usize = self.dimension.0 - self.position.0;
        if space_below >= 4 {
            let slice = (
                self.matrix[self.position.0][self.position.1],
                self.matrix[self.position.0 + 1][self.position.1],
                self.matrix[self.position.0 + 2][self.position.1],
                self.matrix[self.position.0 + 3][self.position.1],
            );
            match slice {
                ('X', 'M', 'A', 'S') => 1,
                ('S', 'A', 'M', 'X') => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
    fn has_forward_diagonal_match(&self) -> i32 {
        let space_to_right: usize = self.dimension.1 - self.position.1;
        let space_below: usize = self.dimension.0 - self.position.0;
        if space_below >= 4 && space_to_right >= 4 {
            let slice = (
                self.matrix[self.position.0][self.position.1],
                self.matrix[self.position.0 + 1][self.position.1 + 1],
                self.matrix[self.position.0 + 2][self.position.1 + 2],
                self.matrix[self.position.0 + 3][self.position.1 + 3],
            );
            match slice {
                ('X', 'M', 'A', 'S') => 1,
                ('S', 'A', 'M', 'X') => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
    fn has_backward_diagonal_match(&self) -> i32 {
        let space_below: usize = self.dimension.0 - self.position.0;
        if space_below >= 4 && self.position.1 >= 3 {
            let slice = (
                self.matrix[self.position.0][self.position.1],
                self.matrix[self.position.0 + 1][self.position.1 - 1],
                self.matrix[self.position.0 + 2][self.position.1 - 2],
                self.matrix[self.position.0 + 3][self.position.1 - 3],
            );
            match slice {
                ('X', 'M', 'A', 'S') => 1,
                ('S', 'A', 'M', 'X') => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
    fn has_x_mas(&self) -> i32 {
        let space_to_right: usize = self.dimension.1 - self.position.1;
        let space_below: usize = self.dimension.0 - self.position.0;
        if space_below >= 3 && space_to_right >= 3 {
            let slice = (
                self.matrix[self.position.0][self.position.1],
                self.matrix[self.position.0 + 1][self.position.1 + 1],
                self.matrix[self.position.0 + 2][self.position.1 + 2],
                self.matrix[self.position.0][self.position.1 + 2],
                self.matrix[self.position.0 + 2][self.position.1],
            );
            match slice {
                ('M', 'A', 'S', 'M', 'S') => 1,
                ('M', 'A', 'S', 'S', 'M') => 1,
                ('S', 'A', 'M', 'M', 'S') => 1,
                ('S', 'A', 'M', 'S', 'M') => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
}

// There's probably a better way of doing paths...
fn make_crossword_matrix(path: String) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(matrix)
}

pub fn part1(no_print: bool) -> i64 {
    let matrix = make_crossword_matrix("src/data/day4.txt".to_string());

    let m = matrix.unwrap();
    let mut matches: Vec<i32> = Vec::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let element = CrosswordElement {
                position: (i, j),
                dimension: (m[i].len(), m.len()),
                matrix: m.clone(),
            };
            matches.push(element.has_horisontal_match());
            matches.push(element.has_vertical_match());
            matches.push(element.has_forward_diagonal_match());
            matches.push(element.has_backward_diagonal_match());
        }
    }

    let result: i64 = matches.iter().sum::<i32>() as i64;

    if !no_print {
        println!("Part 1: {}", result);
    }

    result
}

pub fn part2(no_print: bool) -> i64 {
    let matrix = make_crossword_matrix("src/data/day4_pt2.txt".to_string());

    let m = matrix.unwrap();
    let mut matches: Vec<i32> = Vec::new();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let element = CrosswordElement {
                position: (i, j),
                dimension: (m[i].len(), m.len()),
                matrix: m.clone(),
            };
            matches.push(element.has_x_mas());
        }
    }

    let result: i64 = matches.iter().sum::<i32>() as i64;

    if !no_print {
        println!("Part 2: {}", result);
    }

    result
}
