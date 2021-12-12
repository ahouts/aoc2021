use std::error::Error;
use std::io::BufReader;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day4.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day4::part1(BufReader::new(f)));
    Ok(())
}
