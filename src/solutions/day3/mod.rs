use crate::utils::read_lines;

const INPUT: &str = "data/day3/input";

#[allow(dead_code)]
pub fn solve_day3_part1() -> i32 {
    let n = "010100110100".len();
    let mut bits_cnts: Vec<i32> = vec![0; n];

    let mut gamma: i32 = 0;
    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(line) = line {
                for (i, c) in line.chars().enumerate() {
                    match c {
                        '1' => bits_cnts[i] += 1,
                        _ => bits_cnts[i] -= 1,
                    };
                }
            }
        }
    }
    for bit in bits_cnts {
        match bit {
            b if b < 0 => gamma = (gamma << 1) + 0,
            b if b > 0 => gamma = (gamma << 1) + 1,
            _ => {}
        }
    }

    println!("gamma: {:#020b}", gamma);

    gamma * ((1 << n) - 1 - gamma)
}

#[allow(dead_code)]
pub fn solve_day3_part2() -> i32 {
    let n = "010100110100".len();

    let mut nums: Vec<i32> = vec![];
    let (mut ones, mut zeros): (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    if let Ok(lines) = read_lines(INPUT) {
        for line in lines {
            if let Ok(line) = line {
                nums.push(isize::from_str_radix(&line, 2).unwrap() as i32);
            }
        }
    }
    let mut nums2 = nums.clone();
    let mut i = 0;
    let oxygen: i32;

    while nums.len() > 1 {
        if i > n - 1 {
            println!("wrong");
            break;
        }
        // count
        for num in nums.iter() {
            if num & (1 << (n - i - 1)) == 0 {
                zeros.push(*num);
            } else {
                ones.push(*num);
            }
        }
        if ones.len() >= zeros.len() {
            nums = ones.clone();
        } else {
            nums = zeros.clone();
        }
        ones.clear();
        zeros.clear();
        i += 1;
    }
    oxygen = nums[0];

    // find co2

    let co2: i32;
    i = 0;
    while nums2.len() > 1 {
        if i > n - 1 {
            println!("wrong");
            break;
        }
        // count
        for num in nums2.iter() {
            if num & (1 << (n - i - 1)) == 0 {
                zeros.push(*num);
            } else {
                ones.push(*num);
            }
        }
        if zeros.len() <= ones.len() {
            nums2 = zeros.clone();
        } else {
            nums2 = ones.clone();
        }
        ones.clear();
        zeros.clear();
        i += 1;
    }
    co2 = nums2[0];

    println!("Got oxygen: {}, co2: {}", oxygen, co2);

    oxygen * co2
}
