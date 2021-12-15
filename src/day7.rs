use std::io::BufRead;

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> i32 {
    let positions: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    (positions.iter().copied().min().unwrap()..positions.iter().copied().max().unwrap())
        .map(|p| positions.iter().copied().map(|s| i32::abs(s - p)).sum())
        .min()
        .unwrap()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> i32 {
    let positions: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    (positions.iter().copied().min().unwrap()..positions.iter().copied().max().unwrap())
        .map(|p| positions.iter().copied().map(|s| dist_to_fuel(s - p)).sum())
        .min()
        .unwrap()
}

fn dist_to_fuel(dist: i32) -> i32 {
    (1..=i32::abs(dist)).sum()
}
