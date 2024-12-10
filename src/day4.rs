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
    fn has_horisontal_xmas(&self) -> i32 {
        let space_to_right: usize = self.dimension.0 - self.position.0;
        if space_to_right >= 4 {
            let slice = (
                self.matrix[self.position.0][self.position.1],
                self.matrix[self.position.0 + 1][self.position.1],
                self.matrix[self.position.0 + 2][self.position.1],
                self.matrix[self.position.0 + 3][self.position.1],
            );
            match slice {
                ('X', 'M', 'A', 'S') => 1,
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

pub fn part1() {
    let matrix = make_crossword_matrix("src/data/day4_test.txt".to_string());

    let m = matrix.unwrap();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let element = CrosswordElement {
                position: (i.try_into().unwrap(), j.try_into().unwrap()),
                dimension: (m[i].len().try_into().unwrap(), m.len().try_into().unwrap()),
                matrix: m.clone(),
            };
            println!("{}", element.has_horisontal_xmas());
        }
    }
}
