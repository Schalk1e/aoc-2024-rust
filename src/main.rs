use clap::{Parser, Subcommand};
use markdown_table_formatter::format_tables;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

/// Advent of Code CLI
///
/// This command-line interface (CLI) allows you to run specific Advent of Code
/// challenges by day, as well as generate a table summarizing results.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Aoc {
    /// The subcommand to execute
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available subcommands for the CLI
#[derive(Subcommand)]
enum Commands {
    /// Run a specific day's challenge
    Daily {
        /// The day to run (1-9)
        #[arg(short, long)]
        day: i32,
    },
    /// Generate a Markdown table summarizing results
    ///
    /// **Note:** This command may take a while to execute, as it runs each day's
    /// computations in summary mode.
    Table {},
}

fn main() {
    let cli = Aoc::parse();

    match &cli.command {
        Some(Commands::Daily { day }) => match day {
            1 => {
                day1::part1(false);
                day1::part2(false);
            }
            2 => {
                day2::part1(false);
                day2::part2(false);
            }
            3 => {
                day3::part1(false);
                day3::part2(false);
            }
            4 => {
                day4::part1(false);
                day4::part2(false);
            }
            5 => {
                day5::part1(false);
                day5::part2(false);
            }
            6 => {
                day6::part1(false);
                day6::part2(false);
            }
            7 => {
                day7::part1(false);
                day7::part2(false);
            }
            8 => {
                day8::part1(false);
                day8::part2(false);
            }
            9 => {
                day9::part1(false);
            }
            _ => {
                println!("Hold on! We haven't completed that day yet :)")
            }
        },
        Some(Commands::Table {}) => {
            println!("NOTE: Day 6, Part 2 takes a while... Excluding from table for now.");
            let data = [
                vec![day1::part1(true), day1::part2(true)],
                vec![day2::part1(true), day2::part2(true)],
                vec![day3::part1(true), day3::part2(true)],
                vec![day4::part1(true), day4::part2(true)],
                vec![day5::part1(true), day5::part2(true)],
                vec![day6::part1(true), 123],
                vec![day7::part1(true), day7::part2(true)],
                vec![day8::part1(true), day8::part2(true)],
                vec![day9::part1(true)],
            ];
            let mut table = String::from("| Day | Part 1 | Part 2 |\n| :-- | :--: | --: |\n");

            for (day, values) in data.iter().enumerate() {
                if values.len() == 2 {
                    table.push_str(&format!(
                        "|  {}  |   {}   |   {}   |\n",
                        // Index 0 is day 1...
                        day + 1,
                        values[0],
                        values[1]
                    ))
                } else {
                    table.push_str(&format!("|  {}  |   ?   |   ?   |\n", day + 1));
                };
            }

            let formatted_table = format_tables(table);

            println!("{}", formatted_table);
        }
        None => {}
    }
}
