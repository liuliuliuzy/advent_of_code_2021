// declare it in main.rs first
mod solutions;
mod utils;

use crate::solutions::day7::solve_day7_part2;

#[allow(dead_code)]
fn solve(x: i32) -> i32 {
    match x {
        y if y < 40 => 40 - y,
        _ => 0,
    }
}

fn main() {
    println!("Hello, advent of code!");
    println!("The answer is: {}", solve_day7_part2());
    // try_case();
    println!("{}", 3i32 / 2i32);
}
