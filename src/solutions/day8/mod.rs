#[allow(dead_code)]
pub fn solve_day8_part1() -> i32 {
    include_str!("../../../data/day8/input")
        .lines()
        .map(|line| {
            let (_, right) = line.split_once('|').unwrap();
            // println!("{}", right);
            right
                .split_ascii_whitespace()
                .filter(|p| {
                    // println!("p.len(): {}", p.len());
                    // p.len() == 2 || p.len() == 3 || p.len() == 4 || p.len() == 7
                    matches!(p.len(), 2 | 3 | 4 | 7)
                })
                .count() as i32
        })
        .sum::<i32>()
}

#[allow(dead_code)]
pub fn solve_day8_part2() -> u32 {
    include_bytes!("../../../data/day8/input")
        // .lines()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut part = line.splitn(2, |&b| b == b'|'); // 创建迭代器，提取输入的两部分
            let mut input = part.next().unwrap().split(|&b| b == b' '); // 创建迭代器，提取第一部分输入的10个pattern
            let one = input.clone().find(|d| d.len() == 2).unwrap(); // find会更改迭代器的状态，所以这里需要一个clone
            let four = input.find(|d| d.len() == 4).unwrap();
            part.next()
                .unwrap()
                .split(|&b| b == b' ')
                .skip(1)
                .map(|d| match d.len() {
                    // 独特的点亮灯管数量对应的数字映射
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    len => match (
                        len,
                        d.iter().filter(|&b| one.contains(b)).count(),
                        d.iter().filter(|&b| four.contains(b)).count(),
                    ) {
                        (5, 1, 3) => 5,
                        (5, 2, 3) => 3,
                        (5, _, 2) => 2,
                        (6, 1, _) => 6,
                        (6, _, 3) => 0,
                        (6, _, 4) => 9,
                        _ => unreachable!(),
                    },
                })
                .enumerate() // 迭代元素再加个index
                .fold(0, |acc, (i, n)| acc + n * 10_u32.pow(3 - i as u32))
        })
        .sum::<u32>()
}
