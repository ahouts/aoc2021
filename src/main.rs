use std::error::Error;
use std::io::BufReader;

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day2.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day2::part2(BufReader::new(f)));
    Ok(())
}
