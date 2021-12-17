use std::collections::HashSet;
use std::io::BufRead;

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> usize {
    let mut scratch = Vec::<(u16, u16)>::new();
    let mut grid = HashSet::<(u16, u16)>::new();
    let mut phase_two = false;
    for res in reader.lines() {
        let line = res.unwrap();
        if line.is_empty() {
            phase_two = true;
            continue;
        }
        if !phase_two {
            let (x_text, y_text) = line.split_once(',').unwrap();
            let x = x_text.parse().unwrap();
            let y = y_text.parse().unwrap();
            grid.insert((x, y));
        } else {
            let eq = line.split(' ').skip(2).next().unwrap();
            let (axis, val_text) = eq.split_once('=').unwrap();
            let val: u16 = val_text.parse().unwrap();
            match axis {
                "x" => {
                    grid.iter()
                        .copied()
                        .filter(|(x, _)| *x > val)
                        .for_each(|coord| scratch.push(coord));
                    scratch.iter().for_each(|coord| {
                        grid.remove(coord);
                    });
                    for (x, y) in scratch.drain(..) {
                        grid.insert((val - (x - val), y));
                    }
                }
                "y" => {
                    grid.iter()
                        .copied()
                        .filter(|(_, y)| *y > val)
                        .for_each(|coord| scratch.push(coord));
                    scratch.iter().for_each(|coord| {
                        grid.remove(coord);
                    });
                    for (x, y) in scratch.drain(..) {
                        grid.insert((x, val - (y - val)));
                    }
                }
                _ => panic!(),
            }
            return grid.len();
        }
    }

    unimplemented!()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> String {
    let mut scratch = Vec::<(u16, u16)>::new();
    let mut grid = HashSet::<(u16, u16)>::new();
    let mut phase_two = false;
    for res in reader.lines() {
        let line = res.unwrap();
        if line.is_empty() {
            phase_two = true;
            continue;
        }
        if !phase_two {
            let (x_text, y_text) = line.split_once(',').unwrap();
            let x = x_text.parse().unwrap();
            let y = y_text.parse().unwrap();
            grid.insert((x, y));
        } else {
            let eq = line.split(' ').skip(2).next().unwrap();
            let (axis, val_text) = eq.split_once('=').unwrap();
            let val: u16 = val_text.parse().unwrap();
            match axis {
                "x" => {
                    grid.iter()
                        .copied()
                        .filter(|(x, _)| *x > val)
                        .for_each(|coord| scratch.push(coord));
                    scratch.iter().for_each(|coord| {
                        grid.remove(coord);
                    });
                    for (x, y) in scratch.drain(..) {
                        grid.insert((val - (x - val), y));
                    }
                }
                "y" => {
                    grid.iter()
                        .copied()
                        .filter(|(_, y)| *y > val)
                        .for_each(|coord| scratch.push(coord));
                    scratch.iter().for_each(|coord| {
                        grid.remove(coord);
                    });
                    for (x, y) in scratch.drain(..) {
                        grid.insert((x, val - (y - val)));
                    }
                }
                _ => panic!(),
            }
        }
    }

    let max_x = grid.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = grid.iter().map(|(_, y)| *y).max().unwrap();

    let mut out = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if grid.contains(&(x, y)) {
                out.push('█');
            } else {
                out.push(' ');
            }
        }
        out.push('\n');
    }

    out
}
