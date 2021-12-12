use std::io::BufRead;
use std::ops::{Index, IndexMut};

struct Board {
    data: [[u8; 5]; 5],
}

impl Board {
    fn load(iter: &mut impl Iterator<Item = String>) -> Self {
        let mut board = Board { data: [[0; 5]; 5] };
        fn load_line(board: &mut Board, row: usize, mut iter: &mut impl Iterator<Item = String>) {
            board.data[row]
                .iter_mut()
                .zip(
                    iter.next()
                        .unwrap()
                        .split_whitespace()
                        .map(|n| n.parse::<u8>().unwrap()),
                )
                .for_each(|(pos, num)| *pos = num);
        }
        for i in 0..5 {
            load_line(&mut board, i, iter);
        }
        board
    }

    fn completion_score(&self, nums: &[u8]) -> Option<u32> {
        let mut complete = false;
        for i in 0..5 {
            if self.data[i].iter().copied().all(|n| nums.contains(&n)) {
                complete = true;
                break;
            }
        }
        for i in 0..5 {
            if self.data.iter().map(|r| r[i]).all(|n| nums.contains(&n)) {
                complete = true;
                break;
            }
        }

        if !complete {
            return None;
        }

        Some(
            self.data
                .iter()
                .flat_map(|r| r.iter().copied())
                .filter(|n| !nums.contains(n))
                .map(|n| n as u32)
                .sum(),
        )
    }
}

impl Index<(usize, usize)> for Board {
    type Output = u8;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}

pub fn part1<R: BufRead>(reader: R) -> u32 {
    let mut lines = reader.lines();
    let nums: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut lines = lines.map(|res| res.unwrap());
    let mut boards = Vec::new();
    while let Some(_) = lines.next() {
        let board = Board::load(&mut lines);
        boards.push(board);
    }

    for i in 1..nums.len() {
        for b in boards.iter() {
            if let Some(score) = b.completion_score(&nums[..i]) {
                return score * nums[i - 1] as u32;
            }
        }
    }

    panic!()
}
