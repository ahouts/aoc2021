use std::error::Error;
use std::io::BufReader;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new("day1.txt");
    let f = std::fs::File::open(p)?;
    println!("{}", day1::part2(BufReader::new(f)));
    Ok(())
}
