use std::io::BufRead;

pub fn part1<R: BufRead>(reader: R) -> usize {
    let depths: Vec<u16> = reader.lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u16>())
        .map(Result::unwrap)
        .collect();
    depths.iter().copied()
        .zip(depths.iter().copied().skip(1))
        .filter(|(p, n)| n > p)
        .count()
}

pub fn part2<R: BufRead>(reader: R) -> usize {
    let depths: Vec<u16> = reader.lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u16>())
        .map(Result::unwrap)
        .collect();
    let w1 = depths.iter().copied()
        .zip(depths.iter().copied().skip(1))
        .zip(depths.iter().copied().skip(2))
        .map(|((a, b), c)| a + b + c);
    let w2 = w1.clone().skip(1);
    w1.zip(w2)
        .filter(|(p, n)| n > p)
        .count()
}