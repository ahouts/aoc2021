use std::collections::HashMap;
use std::io::BufRead;

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u32 {
    let grid: Vec<Vec<u8>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| line.bytes().map(|c| c - b'0').collect())
        .collect();

    (0..grid.len())
        .flat_map(|row| (0..grid[row].len()).map(move |col| (row, col)))
        .filter(|(row, col)| {
            let row = *row as isize;
            let col = *col as isize;
            let value_of = |row: isize, col: isize| -> u8 {
                if row < 0 || col < 0 {
                    u8::MAX
                } else {
                    grid.get(row as usize)
                        .map(|r| r.get(col as usize))
                        .flatten()
                        .copied()
                        .unwrap_or(u8::MAX)
                }
            };
            let my_val = value_of(row, col);
            my_val < value_of(row + 1, col)
                && my_val < value_of(row - 1, col)
                && my_val < value_of(row, col + 1)
                && my_val < value_of(row, col - 1)
        })
        .map(|(row, col)| grid[row][col] + 1)
        .map(|risk| risk as u32)
        .sum()
}

fn move_down(board: &Vec<Vec<u8>>, row: usize, col: usize) -> (usize, usize) {
    let value_of = |row: isize, col: isize| -> u8 {
        if row < 0 || col < 0 {
            u8::MAX
        } else {
            board
                .get(row as usize)
                .map(|r| r.get(col as usize))
                .flatten()
                .copied()
                .unwrap_or(u8::MAX)
        }
    };
    let row = row as isize;
    let col = col as isize;
    let my_val = value_of(row, col);
    [
        (row, col),
        (row + 1, col),
        (row - 1, col),
        (row, col + 1),
        (row, col - 1),
    ]
    .into_iter()
    .map(|(row, col)| ((row, col), value_of(row, col)))
    .filter(|(_, value)| *value != my_val)
    .max_by_key(|(_, value)| *value)
    .map(|((row, col), _)| (row as usize, col as usize))
    .unwrap_or((row as usize, col as usize))
}

fn find_basin(board: &Vec<Vec<u8>>, row: usize, col: usize) -> (usize, usize) {
    let mut current_loc = (row, col);
    loop {
        let new_loc = move_down(board, current_loc.0, current_loc.1);
        if new_loc == current_loc {
            return current_loc;
        }
        current_loc = new_loc;
    }
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u32 {
    let grid: Vec<Vec<u8>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line: String| line.bytes().map(|c| c - b'0').collect())
        .collect();

    let mut basin_sizes: HashMap<(usize, usize), u16> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 9 {
                continue;
            }
            *basin_sizes.entry(find_basin(&grid, row, col)).or_default() += 1;
        }
    }

    unimplemented!()
}
