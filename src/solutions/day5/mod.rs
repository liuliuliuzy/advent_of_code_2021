use std::vec;

use crate::utils::read_lines;

const INPUT: &str = "data/day5/input";
#[allow(dead_code)]
pub fn solve_day5_part1() -> i32 {
    let mut board = vec![vec![0; 1000]; 1000];
    // let (mut x1, mut y1, mut x2, mut y2): (i32, i32, i32, i32) = (0, 0, 0, 0);
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(line) = line {
                let (first_part, second_part) = line.split_once(" -> ").unwrap();
                let ((sx1, sy1), (sx2, sy2)) = (
                    first_part.split_once(',').unwrap(),
                    second_part.split_once(',').unwrap(),
                );
                let (x1, y1, x2, y2): (i32, i32, i32, i32) = (
                    sx1.parse().unwrap(),
                    sy1.parse().unwrap(),
                    sx2.parse().unwrap(),
                    sy2.parse().unwrap(),
                );
                if x1 != x2 && y1 != y2 {
                    continue;
                }
                if x1 == x2 {
                    let (start, end) = (y1.min(y2), y1.max(y2));
                    for j in start..=end {
                        board[x1 as usize][j as usize] += 1;
                    }
                } else {
                    let (start, end) = (x1.min(x2), x1.max(x2));
                    for i in start..=end {
                        board[i as usize][y1 as usize] += 1;
                    }
                }
            }
        }
    }
    let mut res = 0;
    for row in board {
        for num in row {
            if num > 1 {
                res += 1;
            }
        }
    }
    res
}
#[allow(dead_code)]
pub fn solve_day5_part2() -> usize {
    let mut board = vec![vec![0; 1000]; 1000];
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(line) = line {
                let (first_part, second_part) = line.split_once(" -> ").unwrap();
                let ((sx1, sy1), (sx2, sy2)) = (
                    first_part.split_once(',').unwrap(),
                    second_part.split_once(',').unwrap(),
                );
                let (x1, y1, x2, y2): (i32, i32, i32, i32) = (
                    sx1.parse().unwrap(),
                    sy1.parse().unwrap(),
                    sx2.parse().unwrap(),
                    sy2.parse().unwrap(),
                );
                // 分情况讨论
                match ((x1, y1), (x2, y2)) {
                    // match guard
                    ((x1, y1), (x2, y2)) if x1 == x2 && y1 != y2 => {
                        for j in y1.min(y2)..=y1.max(y2) {
                            board[x1 as usize][j as usize] += 1;
                        }
                    }
                    ((x1, y1), (x2, y2)) if y1 == y2 && x1 != x2 => {
                        for i in x1.min(x2)..=x1.max(x2) {
                            board[i as usize][y1 as usize] += 1;
                        }
                    }
                    ((x1, y1), (x2, y2))
                        if x1 != x2 && y1 != y2 && (x1 - x2).abs() == (y1 - y2).abs() =>
                    {
                        if (x1 - x2) * (y1 - y2) > 0 {
                            // 西南-东北方向
                            let (si, sj) = (x1.min(x2), y1.min(y2));
                            for k in 0..=(x1 - x2).abs() {
                                board[(si + k) as usize][(sj + k) as usize] += 1;
                            }
                        } else {
                            // 西北-东南方向
                            let (si, sj) = (x1.min(x2), y1.max(y2));
                            for k in 0..=(x1 - x2).abs() {
                                board[(si + k) as usize][(sj - k) as usize] += 1;
                            }
                        }
                    }
                    (_, _) => continue,
                }
            }
        }
    }
    board
        .iter()
        .map(|r| r.iter().filter(|&n| *n >= 2).count())
        .sum()
}
