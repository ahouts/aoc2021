use std::collections::HashMap;
use std::io::BufRead;

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u16 {
    let mut lines = reader.lines();
    let mut template = lines.next().unwrap().unwrap().into_bytes();
    lines.next().unwrap().unwrap();

    let mut replacements = HashMap::<(u8, u8), u8>::new();
    for res in lines {
        let line = res.unwrap();
        let (from, to) = line.split_once(" -> ").unwrap();
        match (from.as_bytes(), to.as_bytes()) {
            ([a, b], [r]) => {
                replacements.insert((*a, *b), *r);
            }
            _ => panic!(),
        }
    }

    let mut scratch = Vec::new();
    for _ in 0..10 {
        scratch.clear();
        for (a, b) in template
            .iter()
            .copied()
            .zip(template.iter().copied().skip(1))
        {
            scratch.push(a);
            if let Some(i) = replacements.get(&(a, b)) {
                scratch.push(*i);
            }
        }
        scratch.push(template.last().copied().unwrap());
        std::mem::swap(&mut template, &mut scratch);
    }

    let mut frequencies = HashMap::<u8, u16>::new();
    for c in template {
        *frequencies.entry(c).or_default() += 1;
    }

    frequencies.values().max().copied().unwrap() - frequencies.values().min().copied().unwrap()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u64 {
    let mut lines = reader.lines();
    let template = lines.next().unwrap().unwrap().into_bytes();
    lines.next().unwrap().unwrap();

    let mut replacements = HashMap::<(u8, u8), u8>::new();
    for res in lines {
        let line = res.unwrap();
        let (from, to) = line.split_once(" -> ").unwrap();
        match (from.as_bytes(), to.as_bytes()) {
            ([a, b], [r]) => {
                replacements.insert((*a, *b), *r);
            }
            _ => panic!(),
        }
    }

    let mut pair_frequencies = HashMap::<(u8, u8), u64>::new();
    for (a, b) in template
        .iter()
        .copied()
        .zip(template.iter().copied().skip(1))
    {
        *pair_frequencies.entry((a, b)).or_default() += 1;
    }

    let mut next_pair_frequencies = HashMap::<(u8, u8), u64>::new();
    for _ in 0..40 {
        next_pair_frequencies.clear();
        for ((a, b), c) in pair_frequencies.iter() {
            if let Some(r) = replacements.get(&(*a, *b)).copied() {
                *next_pair_frequencies.entry((*a, r)).or_default() += *c;
                *next_pair_frequencies.entry((r, *b)).or_default() += *c;
            } else {
                *next_pair_frequencies.entry((*a, *b)).or_default() += *c;
            }
        }
        std::mem::swap(&mut next_pair_frequencies, &mut pair_frequencies);
    }

    let mut frequencies = HashMap::<u8, u64>::new();
    for (i, c) in pair_frequencies.iter().map(|((a, _), c)| (*a, *c)) {
        *frequencies.entry(i).or_default() += c;
    }

    *frequencies.entry(*template.last().unwrap()).or_default() += 1;

    frequencies.values().max().copied().unwrap() - frequencies.values().min().copied().unwrap()
}
