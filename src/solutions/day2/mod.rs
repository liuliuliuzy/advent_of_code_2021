use crate::utils::read_lines;

const INPUT: &str = "data/day2/input";

#[allow(dead_code)]
pub fn solve_day1_part1() -> i32 {
    let (mut i, mut j): (i32, i32) = (0, 0);
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(line) = line {
                let mut iter = line.split_whitespace();
                let dir = iter.next().unwrap();
                let cnt = iter.next().unwrap().parse::<i32>().unwrap();
                match dir {
                    "up" => i -= cnt,
                    "down" => i += cnt,
                    "forward" => j += cnt,
                    _ => {}
                }
            }
        }
    }
    i * j
}

#[allow(dead_code)]
pub fn solve_day2_part2() -> i32 {
    let (mut i, mut j): (i32, i32) = (0, 0);
    let mut aim_depth = 0;

    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(line) = line {
                let mut iter = line.split_whitespace();
                let dir = iter.next().unwrap();
                let cnt = iter.next().unwrap().parse::<i32>().unwrap();
                match dir {
                    "up" => aim_depth -= cnt,
                    "down" => aim_depth += cnt,
                    "forward" => {
                        j += cnt;
                        i += cnt * aim_depth
                    }
                    _ => {} // '{}' means do nothing
                }
            }
        }
    }

    i * j
}
