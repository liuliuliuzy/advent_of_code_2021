// declare it in main.rs first
// 其它mod的根从main crate开始
mod solutions;
mod utils;

use crate::solutions::day8::solve_day8_part2;

#[allow(dead_code)]
fn solve(x: i32) -> i32 {
    match x {
        y if y < 40 => 40 - y,
        _ => 0,
    }
}

fn main() {
    println!("Hello, advent of code!");
    println!("The answer is: {}", solve_day8_part2());
    // try_case();
}
