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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Aoc {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Daily {
        // The day to run.
        #[arg(short, long)]
        day: i32,
    },
    Table {
        // No args needed here
    },
}

fn main() {
    let cli = Aoc::parse();

    match &cli.command {
        Some(Commands::Daily { day }) => match day {
            1 => {
                day1::part1();
                day1::part2();
            }
            2 => {
                day2::part1();
                day2::part2();
            }
            3 => {
                day3::part1();
                day3::part2();
            }
            4 => {
                day4::part1();
                day4::part2();
            }
            5 => {
                day5::part1(false);
                day5::part2();
            }
            6 => {
                day6::part1();
                day6::part2();
            }
            7 => {
                day7::part1();
                day7::part2();
            }
            8 => {
                day8::part1();
                day8::part2();
            }
            9 => {
                day9::part1();
            }
            _ => {
                println!("Hold on! We haven't completed that day yet :)")
            }
        },
        Some(Commands::Table {}) => {
            let data = vec![
                vec![day1::part1(), day1::part2()],
                vec![day2::part1(), day2::part2()],
                vec![day3::part1(), day3::part2()],
                vec![day4::part1(), day4::part2()],
                vec![day5::part1(false), day5::part2()],
                vec![day6::part1(), day6::part2()],
                vec![day7::part1(), day7::part2()],
                vec![day8::part1(), day8::part2()],
                vec![day9::part1()],
            ];
            let mut table = String::from("| Day | Part 1 | Part 2 |\n| :-- | :--: | --: |\n");

            for (day, values) in data.iter().enumerate() {
                if values.len() == 2 {
                    table.push_str(&format!(
                        "|  {}  |   {}   |   {}   |\n",
                        day, values[0], values[1]
                    ))
                } else {
                    table.push_str(&format!("|  {}  |   ?   |   ?   |\n", day + 1));
                };
            }

            let formatted_table = format_tables(table);

            println!("{:?}", formatted_table);
        }
        None => {}
    }
}
