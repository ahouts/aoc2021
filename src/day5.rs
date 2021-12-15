use std::io::BufRead;
use std::ops::{Index, IndexMut};

#[derive(Default)]
struct Board {
    data: Vec<Vec<u8>>,
}

impl Board {
    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.grow_to_fit(x1, y1);
        self.grow_to_fit(x2, y2);

        match (x1 == x2, y1 == y2) {
            (true, false) => {
                let ymin = usize::min(y1, y2);
                let ymax = usize::max(y1, y2);
                for y in ymin..=ymax {
                    self[(x1, y)] += 1;
                }
            }
            (false, true) => {
                let xmin = usize::min(x1, x2);
                let xmax = usize::max(x1, x2);
                for x in xmin..=xmax {
                    self[(x, y1)] += 1;
                }
            }
            _ => {
                let mut p1 = (x1, y1);
                let mut p2 = (x2, y2);
                if p1.0 > p2.0 {
                    std::mem::swap(&mut p1, &mut p2);
                }
                let update_point: fn((usize, usize)) -> (usize, usize) = if p1.1 < p2.1 {
                    |p| (p.0 + 1, p.1 + 1)
                } else {
                    |p| (p.0 + 1, p.1 - 1)
                };
                while p1 != p2 {
                    self[p1] += 1;
                    p1 = update_point(p1);
                }
                self[p1] += 1;
            }
        }
    }

    fn grow_to_fit(&mut self, x: usize, y: usize) {
        while self.data.len() <= x {
            self.data
                .push(vec![0; self.data.get(0).map(|r| r.len()).unwrap_or(0)]);
        }

        while self.data[0].len() <= y {
            for i in 0..self.data.len() {
                self.data[i].push(0);
            }
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x][y]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y]
    }
}

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> usize {
    let mut board = Board::default();
    for res in reader.lines() {
        let line = res.unwrap();
        let parts: Vec<_> = line.split(" -> ").collect();
        match parts.as_slice() {
            [p1, p2] => {
                let d1: Vec<_> = p1.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                let d2: Vec<_> = p2.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                match (d1.as_slice(), d2.as_slice()) {
                    ([x1, y1], [x2, y2]) => {
                        if *x1 == *x2 || *y1 == *y2 {
                            board.draw_line(*x1, *y1, *x2, *y2);
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    board
        .data
        .iter()
        .flat_map(|r| r.iter().copied())
        .filter(|n| *n > 1)
        .count()
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> usize {
    let mut board = Board::default();
    for res in reader.lines() {
        let line = res.unwrap();
        let parts: Vec<_> = line.split(" -> ").collect();
        match parts.as_slice() {
            [p1, p2] => {
                let d1: Vec<_> = p1.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                let d2: Vec<_> = p2.split(",").map(|n| n.parse::<usize>().unwrap()).collect();
                match (d1.as_slice(), d2.as_slice()) {
                    ([x1, y1], [x2, y2]) => board.draw_line(*x1, *y1, *x2, *y2),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    board
        .data
        .iter()
        .flat_map(|r| r.iter().copied())
        .filter(|n| *n > 1)
        .count()
}
