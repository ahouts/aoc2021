use std::error::Error;
use std::io::BufReader;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day6.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day6::part2(BufReader::new(f)));
    Ok(())
}
