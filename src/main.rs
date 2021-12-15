use std::error::Error;
use std::io::BufReader;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day8.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day8::part2(BufReader::new(f)));
    Ok(())
}
