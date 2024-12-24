mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("\nDay 1\n-----\n");
    let _ = day1::part1();
    let _ = day1::part2();
    println!("\nDay 2\n-----\n");
    day2::part1();
    day2::part2();
    println!("\nDay 3\n-----\n");
    day3::part1();
    day3::part2();
    day3::part2_edit();
    println!("\nDay 4\n-----\n");
    day4::part1();
    day4::part2();
    println!("\nDay 5\n-----\n");
    day5::part1(false);
    day5::part2();
    println!("\nDay 6\n-----\n");
    day6::part1();
    // day6::part2();
    println!("Part 2: This takes a long time to run! It's brute force.");
}
