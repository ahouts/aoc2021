use std::io::BufRead;
use std::ops::Add;

struct Pos {
    x: u32,
    y: u32,
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

impl Add<Ins> for Pos {
    type Output = Pos;

    fn add(self, ins: Ins) -> Self::Output {
        match ins {
            Ins::Forward(f) => Pos { x: self.x + f, ..self },
            Ins::Down(f) => Pos { y: self.y + f, ..self },
            Ins::Up(f) => Pos { y: self.y - f, ..self },
        }
    }
}

enum Ins {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn part1<R: BufRead>(reader: R) -> u32 {
    let pos = reader.lines()
        .map(Result::unwrap)
        .map(|line: String| {
            let d: Vec<&str> = line.split_whitespace().collect();
            match d.as_slice() {
                ["forward", f] => Ins::Forward(f.parse::<u32>().unwrap()),
                ["down", f] => Ins::Down(f.parse::<u32>().unwrap()),
                ["up", f] => Ins::Up(f.parse::<u32>().unwrap()),
                _ => panic!(),
            }
        })
        .fold(Pos::default(), |p, i| p + i);
    pos.x * pos.y
}

#[derive(Default)]
struct Pos2 {
    x: i32,
    y: i32,
    aim: i32,
}

impl Add<Ins> for Pos2 {
    type Output = Pos2;

    fn add(self, ins: Ins) -> Self::Output {
        match ins {
            Ins::Forward(f) => Pos2 { x: self.x + f as i32, y: self.y + self.aim * f as i32, ..self },
            Ins::Down(f) => Pos2 { aim: self.aim + f as i32, ..self },
            Ins::Up(f) => Pos2 { aim: self.aim - f as i32, ..self },
        }
    }
}

pub fn part2<R: BufRead>(reader: R) -> i32 {
    let pos = reader.lines()
        .map(Result::unwrap)
        .map(|line: String| {
            let d: Vec<&str> = line.split_whitespace().collect();
            match d.as_slice() {
                ["forward", f] => Ins::Forward(f.parse().unwrap()),
                ["down", f] => Ins::Down(f.parse().unwrap()),
                ["up", f] => Ins::Up(f.parse().unwrap()),
                _ => panic!(),
            }
        })
        .fold(Pos2::default(), |p, i| p + i);
    pos.x * pos.y
}
