mod day1;
mod day2;
mod day3;

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
}
