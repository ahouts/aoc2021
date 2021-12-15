use bit_iter::BitIter;
use bitflags::bitflags;
use std::collections::HashMap;
use std::io::BufRead;

bitflags! {
    struct Segments: u8 {
        const A = 0b0000_0001;
        const B = 0b0000_0010;
        const C = 0b0000_0100;
        const D = 0b0000_1000;
        const E = 0b0001_0000;
        const F = 0b0010_0000;
        const G = 0b0100_0000;
    }
}

impl Segments {
    fn iter(self) -> impl Iterator<Item = Segments> {
        BitIter::from(self.bits).map(|b| Segments::from_bits_truncate((1 << b) as u8))
    }
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn segments(self) -> Segments {
        const A: Segments = Segments::A;
        const B: Segments = Segments::B;
        const C: Segments = Segments::C;
        const D: Segments = Segments::D;
        const E: Segments = Segments::E;
        const F: Segments = Segments::F;
        const G: Segments = Segments::G;
        match self {
            Digit::Zero => A | B | C | E | F | G,
            Digit::One => C | F,
            Digit::Two => A | C | D | E | G,
            Digit::Three => A | C | D | F | G,
            Digit::Four => B | C | D | F,
            Digit::Five => A | B | D | F | G,
            Digit::Six => A | B | D | E | F | G,
            Digit::Seven => A | C | F,
            Digit::Eight => A | B | C | D | E | F | G,
            Digit::Nine => A | B | C | D | F | G,
        }
    }
}

fn parse_line(text: &str) -> Vec<Segments> {
    text.split(' ')
        .map(|p| {
            p.chars()
                .map(|c| match c {
                    'a' => Segments::A,
                    'b' => Segments::B,
                    'c' => Segments::C,
                    'd' => Segments::D,
                    'e' => Segments::E,
                    'f' => Segments::F,
                    'g' => Segments::G,
                    _ => panic!(),
                })
                .fold(Segments::empty(), |s1, s2| s1 | s2)
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> usize {
    reader
        .lines()
        .flat_map(|result| {
            let line = result.unwrap();
            let parts = line.split('|').collect::<Vec<_>>();
            let numbers_text = match parts.as_slice() {
                [_, numbers] => numbers,
                _ => panic!(),
            };
            parse_line(numbers_text).into_iter()
        })
        .filter(|segments| [2, 4, 3, 7].contains(&segments.bits.count_ones()))
        .count()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u32 {
    let mut total = 0;
    for result in reader.lines() {
        let line = result.unwrap();
        let parts = line.split('|').collect::<Vec<_>>();
        let (given_text, numbers_text) = match parts.as_slice() {
            [given, numbers] => (given, numbers),
            _ => panic!(),
        };
        let given = parse_line(given_text);
        let numbers = parse_line(numbers_text);

        let mut segments_to_given: HashMap<u8, Vec<Segments>> = HashMap::new();
        for given_segments in given.into_iter() {
            segments_to_given
                .entry(given_segments.bits.count_ones() as u8)
                .or_default()
                .push(given_segments);
        }

        let one_segments = segments_to_given.get(&2).unwrap()[0];
        let seven_segments = segments_to_given.get(&3).unwrap()[0];
        let four_segments = segments_to_given.get(&4).unwrap()[0];
        let eight_segments = segments_to_given.get(&7).unwrap()[0];

        let zero_six_nine_segments = segments_to_given.get(&6).unwrap();
        let nine_segments = zero_six_nine_segments
            .iter()
            .filter(|segments| segments.intersection(four_segments).bits.count_ones() == 4)
            .next()
            .copied()
            .unwrap();
        let zero_segments = zero_six_nine_segments
            .iter()
            .filter(|segments| **segments != nine_segments)
            .filter(|segments| segments.intersection(one_segments).bits.count_ones() == 2)
            .next()
            .copied()
            .unwrap();
        let six_segments = zero_six_nine_segments
            .iter()
            .filter(|segments| **segments != nine_segments)
            .filter(|segments| **segments != zero_segments)
            .next()
            .copied()
            .unwrap();

        let two_three_five_segments = segments_to_given.get(&5).unwrap();
        let three_segments = two_three_five_segments
            .iter()
            .filter(|segments| segments.intersection(one_segments).bits.count_ones() == 2)
            .next()
            .copied()
            .unwrap();
        let five_segments = two_three_five_segments
            .iter()
            .filter(|segments| **segments != three_segments)
            .filter(|segments| segments.difference(six_segments).bits.count_ones() == 0)
            .next()
            .copied()
            .unwrap();
        let two_segments = two_three_five_segments
            .iter()
            .filter(|segments| **segments != three_segments)
            .filter(|segments| **segments != five_segments)
            .next()
            .copied()
            .unwrap();

        let mut lookup = HashMap::new();
        lookup.insert(zero_segments, Digit::Zero);
        lookup.insert(one_segments, Digit::One);
        lookup.insert(two_segments, Digit::Two);
        lookup.insert(three_segments, Digit::Three);
        lookup.insert(four_segments, Digit::Four);
        lookup.insert(five_segments, Digit::Five);
        lookup.insert(six_segments, Digit::Six);
        lookup.insert(seven_segments, Digit::Seven);
        lookup.insert(eight_segments, Digit::Eight);
        lookup.insert(nine_segments, Digit::Nine);

        let mut number = 0;
        for n in numbers {
            number *= 10;
            match lookup.get(&n).unwrap() {
                Digit::Zero => {}
                Digit::One => number += 1,
                Digit::Two => number += 2,
                Digit::Three => number += 3,
                Digit::Four => number += 4,
                Digit::Five => number += 5,
                Digit::Six => number += 6,
                Digit::Seven => number += 7,
                Digit::Eight => number += 8,
                Digit::Nine => number += 9,
            }
        }

        total += number;
    }

    total
}
