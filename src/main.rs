use std::error::Error;
use std::io::BufReader;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day15.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day15::part2(BufReader::new(f)));
    Ok(())
}
