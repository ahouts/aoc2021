use std::error::Error;
use std::io::BufReader;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day3.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day3::part2(BufReader::new(f)));
    Ok(())
}
