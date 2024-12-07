use ::std::error::Error;
use ::std::fs;
use num_traits::sign::signum;

// Read and process input
fn process(path: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let input: String = fs::read_to_string(path)?;
    let mut reports = Vec::new();

    for line in input.lines() {
        let report = line.split(" ");
        let collection = report.collect::<Vec<&str>>();

        reports.push(collection.into_iter().map(|v| v.to_string()).collect()); // Wow, fairly lonwinded way of taking ownership... Is there a more obvious way of doing this?
    }

    Ok(reports)
}

// Determine whether a report succeeds or fails
fn success_or_fail(report: Vec<String>) -> i32 {
    let mut ind: i32 = 1;
    let int_report: Vec<i32> = report
        .into_iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let first_difference: i32 = int_report[0] - int_report[1];
    let mut index = 0;

    while index < int_report.len() - 1 {
        let difference: i32 = int_report[index] - int_report[index + 1];
        if difference.abs() > 3
            || difference == 0
            || first_difference == 0
            || signum(difference) != signum(first_difference)
        {
            ind = 0;
        };
        index += 1;
    }

    ind
}

// Use violation counter approach to answer part 2
// NOTE: This doesn't work for part 2 of the problem. You need to *omit* elements and pass
// them to success_or_fail one by one to check.
fn _violation_counter(report: Vec<String>) -> Vec<i32> {
    let int_report: Vec<i32> = report
        .into_iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let first_difference: i32 = int_report[0] - int_report[1];
    let mut index = 0;

    let mut violations: Vec<i32> = Vec::new();

    while index < int_report.len() - 1 {
        let difference: i32 = int_report[index] - int_report[index + 1];
        // Now store differences in vector and count violations
        if difference.abs() > 3
            || difference == 0
            || first_difference == 0
            || signum(difference) != signum(first_difference)
        {
            violations.push(1)
        } else {
            violations.push(0)
        };
        index += 1
    }

    violations
}

// Count violations
// NOTE: This doesn't work for part 2 of the problem. You need to *omit* elements and pass
// them to success_or_fail one by one to check.
fn _damper_safe(violations: Vec<i32>) -> i32 {
    let violations: i32 = violations.iter().sum();

    if violations > 1 {
        0
    } else {
        1
    }
}

// Iterator that omits each element once and tests whether it is still unsafe.
// Iterators aren't typically stateful. Index is a bit of a hack.
struct Reports {
    index: usize,
    init: Vec<String>,
}

impl Iterator for Reports {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Aparently it's normal and idiomatic to return a None
        // when an Iterator has reached its termination condition...
        if self.index >= self.init.len() {
            return None;
        }

        let mut report = self.init.clone();

        report.remove(self.index);

        self.index += 1;

        Some(report)
    }
}

// Loop over reports to check each one for success or failure..
pub fn part1() {
    let input = process("src/data/day2.txt".to_string());
    let mut ind: Vec<i32> = Vec::new();

    let binding = input.unwrap();

    for report in binding {
        ind.push(success_or_fail(report));
    }

    let result: i32 = ind.iter().sum();

    println!("Part 1: {}", result);
}

// Loop over reports to check which ones have 1 or less violations..
pub fn part2() {
    let input = process("src/data/day2.txt".to_string());
    let mut ind: Vec<i32> = Vec::new();

    let binding = input.unwrap();

    for report in binding {
        let mut safe: i32;
        let mut damper_safe: Vec<i32> = Vec::new();

        safe = success_or_fail(report.clone());

        // If not safe, test permutations
        if safe == 0 {
            let iter = Reports {
                init: report,
                index: 0,
            };

            // Test safeness for each element in permutation Iterator
            for permutation in iter {
                damper_safe.push(success_or_fail(permutation));
            }

            // Take the max of the safeness classifications for each perm.
            // If it's one in any of the cases, it will be added as safe.
            // Else we'll push the 0.
            if let Some(max) = damper_safe.iter().max() {
                safe = *max
            } else {
                println!("damper safe is empty!")
            };
        }

        // Push safeness.
        ind.push(safe);
    }

    let result: i32 = ind.iter().sum();

    println!("Part 2: {}", result);
}
