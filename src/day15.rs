use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::BufRead;

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u16 {
    let grid: Vec<Vec<u8>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let goal = ((grid.len() - 1) as i8, (grid[0].len() - 1) as i8);

    #[derive(Debug, Clone)]
    struct Path {
        total_risk: u16,
        x: i8,
        y: i8,
    }

    impl PartialEq for Path {
        fn eq(&self, other: &Self) -> bool {
            self.total_risk == other.total_risk
        }
    }

    impl Eq for Path {}

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.total_risk.partial_cmp(&self.total_risk)
        }
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            other.total_risk.cmp(&self.total_risk)
        }
    }

    let mut options = BinaryHeap::new();
    options.push(Path {
        total_risk: grid[0][1] as u16,
        x: 0,
        y: 1,
    });
    options.push(Path {
        total_risk: grid[1][0] as u16,
        x: 1,
        y: 0,
    });
    let mut already_considered = HashSet::new();
    already_considered.insert((0, 0));
    already_considered.insert((1, 0));
    already_considered.insert((0, 1));

    loop {
        let path = options.pop().unwrap();
        let loc = (path.x, path.y);

        if loc == goal {
            return path.total_risk;
        }

        for (xo, yo) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            let nx = path.x + xo;
            let ny = path.y + yo;
            if nx < 0
                || ny < 0
                || nx > goal.0
                || ny > goal.1
                || already_considered.contains(&(nx, ny))
            {
                continue;
            }
            already_considered.insert((nx, ny));
            options.push(Path {
                total_risk: path.total_risk + grid[nx as usize][ny as usize] as u16,
                x: nx,
                y: ny,
            });
        }
    }
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u16 {
    let mut grid: Vec<Vec<u8>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let orig_x = grid.len();
    let orig_y = grid[0].len();

    for i in 0..4 {
        for j in 0..orig_x {
            grid.push(
                grid[orig_x * i + j]
                    .iter()
                    .copied()
                    .map(|cell| cell + 1)
                    .map(|cell| if cell > 9 { 1 } else { cell })
                    .collect(),
            )
        }
    }

    let mut scratch = Vec::new();
    for i in 0..4 {
        for row in grid.iter_mut() {
            for cell in row[(orig_y * i)..(orig_y * (i + 1))].iter().copied() {
                scratch.push(if cell + 1 > 9 { 1 } else { cell + 1 });
            }
            for cell in scratch.drain(..) {
                row.push(cell);
            }
        }
    }

    let goal = ((grid.len() - 1) as i16, (grid[0].len() - 1) as i16);

    #[derive(Debug, Clone)]
    struct Path {
        total_risk: u16,
        x: i16,
        y: i16,
    }

    impl PartialEq for Path {
        fn eq(&self, other: &Self) -> bool {
            self.total_risk == other.total_risk
        }
    }

    impl Eq for Path {}

    impl PartialOrd for Path {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.total_risk.partial_cmp(&self.total_risk)
        }
    }

    impl Ord for Path {
        fn cmp(&self, other: &Self) -> Ordering {
            other.total_risk.cmp(&self.total_risk)
        }
    }

    let mut options = BinaryHeap::new();
    options.push(Path {
        total_risk: grid[0][1] as u16,
        x: 0,
        y: 1,
    });
    options.push(Path {
        total_risk: grid[1][0] as u16,
        x: 1,
        y: 0,
    });
    let mut already_considered = HashSet::new();
    already_considered.insert((0, 0));
    already_considered.insert((1, 0));
    already_considered.insert((0, 1));

    loop {
        let path = options.pop().unwrap();
        let loc = (path.x, path.y);

        if loc == goal {
            return path.total_risk;
        }

        for (xo, yo) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            let nx = path.x + xo;
            let ny = path.y + yo;
            if nx < 0
                || ny < 0
                || nx > goal.0
                || ny > goal.1
                || already_considered.contains(&(nx, ny))
            {
                continue;
            }
            already_considered.insert((nx, ny));
            options.push(Path {
                total_risk: path.total_risk + grid[nx as usize][ny as usize] as u16,
                x: nx,
                y: ny,
            });
        }
    }
}
