use std::io::BufRead;

fn str_to_bool_vec(text: &str) -> Vec<bool> {
    text.chars().map(|c| c == '1').collect::<Vec<_>>()
}

fn bits_to_u32(bits: Vec<bool>) -> u32 {
    let mut n = 0;
    for b in bits {
        n <<= 1;
        if b {
            n += 1;
        }
    }
    n
}

pub fn part1<R: BufRead>(reader: R) -> u32 {
    let (count, pip_count) = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| str_to_bool_vec(line.as_str()))
        .fold((0, Vec::new()), |(count, mut tot), n| {
            while tot.len() < n.len() {
                tot.push(0);
            }

            for (p, v) in tot.iter_mut().zip(n) {
                if v {
                    *p += 1;
                }
            }

            (count + 1, tot)
        });

    let gamma_bits: Vec<bool> = pip_count.into_iter().map(|c| c > count / 2).collect();
    let epsilon_bits: Vec<bool> = gamma_bits.iter().copied().map(|b| !b).collect();

    bits_to_u32(gamma_bits) * bits_to_u32(epsilon_bits)
}

pub fn part2<R: BufRead>(reader: R) -> u32 {
    let all_bits: Vec<Vec<bool>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| str_to_bool_vec(line.as_str()))
        .collect();

    let mut o2_gen_rating = all_bits.clone();
    let mut pos = 0;
    while o2_gen_rating.len() > 1 {
        let (ones, zeroes) =
            o2_gen_rating.iter().fold(
                (0, 0),
                |(o, z), b| if b[pos] { (o + 1, z) } else { (o, z + 1) },
            );
        if ones >= zeroes {
            o2_gen_rating = o2_gen_rating.into_iter().filter(|r| r[pos]).collect();
        } else {
            o2_gen_rating = o2_gen_rating.into_iter().filter(|r| !r[pos]).collect();
        }
        pos += 1;
    }

    let mut co2_scrub_rating = all_bits;
    pos = 0;
    while co2_scrub_rating.len() > 1 {
        let (ones, zeroes) =
            co2_scrub_rating.iter().fold(
                (0, 0),
                |(o, z), b| if b[pos] { (o + 1, z) } else { (o, z + 1) },
            );
        if ones < zeroes {
            co2_scrub_rating = co2_scrub_rating.into_iter().filter(|r| r[pos]).collect();
        } else {
            co2_scrub_rating = co2_scrub_rating.into_iter().filter(|r| !r[pos]).collect();
        }
        pos += 1;
    }

    bits_to_u32(o2_gen_rating.into_iter().next().unwrap())
        * bits_to_u32(co2_scrub_rating.into_iter().next().unwrap())
}
