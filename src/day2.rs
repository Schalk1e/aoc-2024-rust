use ::std::error::Error;
use ::std::fs;

// Read and process input
fn process(path: String) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let result = Vec::new();
    let input: String = fs::read_to_string(path)?;
    let mut reports = Vec::new();

    for line in input.lines() {
        let report = line.split(" ");
        let collection = report.collect::<Vec<&str>>();

        reports.push(collection);
    }

    for report in reports {
        println!("{}", report.len());
    }

    Ok(result)
}

pub fn part1() {
    let _ = process("src/data/day2.txt".to_string());
}
