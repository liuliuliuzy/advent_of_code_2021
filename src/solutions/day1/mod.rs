use crate::utils::read_lines;

const INPUT: &str = "data/day1/input";

#[allow(dead_code)]
pub fn solve_part1() -> i32 {
    let lines = read_lines(INPUT)
        .unwrap()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u16>>();
    let mut idx = 0;
    let mut res = 0;
    while idx + 1 < lines.len() {
        if lines[idx + 1] > lines[idx] {
            res += 1;
        }
        idx += 1;
    }
    res
}

#[allow(dead_code)]
pub fn solve_part2() -> i32 {
    let lines = read_lines(INPUT)
        .unwrap()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<u16>>();
    let mut idx = 0;
    let mut res = 0;
    while idx + 3 < lines.len() {
        if lines[idx + 3] > lines[idx] {
            res += 1;
        }
        idx += 1;
    }
    res
}
