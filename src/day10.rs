use std::io::BufRead;
use Bracket::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u32 {
    let mut score = 0;
    for result in reader.lines() {
        let line = result.unwrap();
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' => stack.push(Round),
                '[' => stack.push(Square),
                '{' => stack.push(Curly),
                '<' => stack.push(Angle),
                ')' => {
                    if stack.last() == Some(&Round) {
                        stack.pop();
                    } else {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    if stack.last() == Some(&Square) {
                        stack.pop();
                    } else {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    if stack.last() == Some(&Curly) {
                        stack.pop();
                    } else {
                        score += 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.last() == Some(&Angle) {
                        stack.pop();
                    } else {
                        score += 25137;
                        break;
                    }
                }
                _ => panic!(),
            }
        }
    }

    score
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u64 {
    let mut scores = Vec::new();
    for result in reader.lines() {
        let line = result.unwrap();
        let mut stack = Vec::new();

        let mut invalid = false;
        for c in line.chars() {
            match c {
                '(' => stack.push(Round),
                '[' => stack.push(Square),
                '{' => stack.push(Curly),
                '<' => stack.push(Angle),
                ')' => {
                    if stack.last() == Some(&Round) {
                        stack.pop();
                    } else {
                        invalid = true;
                        break;
                    }
                }
                ']' => {
                    if stack.last() == Some(&Square) {
                        stack.pop();
                    } else {
                        invalid = true;
                        break;
                    }
                }
                '}' => {
                    if stack.last() == Some(&Curly) {
                        stack.pop();
                    } else {
                        invalid = true;
                        break;
                    }
                }
                '>' => {
                    if stack.last() == Some(&Angle) {
                        stack.pop();
                    } else {
                        invalid = true;
                        break;
                    }
                }
                _ => panic!(),
            }
        }

        if invalid {
            continue;
        }

        let mut score = 0;
        for elem in stack.into_iter().rev() {
            score *= 5;
            score += match elem {
                Round => 1,
                Square => 2,
                Curly => 3,
                Angle => 4,
            }
        }
        scores.push(score);
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
}
