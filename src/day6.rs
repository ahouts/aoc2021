use std::io::BufRead;

pub fn part1<R: BufRead>(reader: R) -> u32 {
    let line = reader.lines().next().unwrap().unwrap();
    let mut counts = [0; 9];
    line.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| {
            counts[n] += 1;
        });

    for _ in 0..80 {
        let mut new_counts = [0; 9];
        for i in 0..8 {
            new_counts[i] += counts[i + 1];
        }
        new_counts[6] += counts[0];
        new_counts[8] += counts[0];

        std::mem::swap(&mut new_counts, &mut counts);
    }

    counts.iter().sum()
}

pub fn part2<R: BufRead>(reader: R) -> u64 {
    let line = reader.lines().next().unwrap().unwrap();
    let mut counts = [0_u64; 9];
    line.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| {
            counts[n] += 1;
        });

    for _ in 0..256 {
        let mut new_counts = [0; 9];
        for i in 0..8 {
            new_counts[i] += counts[i + 1];
        }
        new_counts[6] += counts[0];
        new_counts[8] += counts[0];

        std::mem::swap(&mut new_counts, &mut counts);
    }

    counts.iter().sum()
}
