// use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq)]
struct Item {
    num: i32,
    cnt: i32,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cnt == other.cnt {
            return Some(other.num.cmp(&self.num));
        }
        Some(other.cnt.cmp(&self.cnt))
    }
}

#[allow(dead_code)]
pub fn solve_day7_part1() -> i32 {
    // let nums: Vec<i32> = include_str!("../../../data/day7/input")
    //     .split(',')
    //     .map(|n| n.parse().unwrap())
    //     .collect();

    // let mut h: HashMap<i32, i32> = HashMap::new();
    // for num in &nums {
    //     *h.entry(*num).or_insert(0) += 1;
    // }

    // let mut cnts: Vec<Item> = h
    //     .iter()
    //     .map(|(key, value)| Item {
    //         num: *key,
    //         cnt: *value,
    //     })
    //     .collect();

    // cnts.sort();

    // let mut i = 0;
    // while i < cnts.len() && cnts[i].cnt == cnts[0].cnt {
    //     i += 1;
    // }

    // let base = cnts[i / 2].num;

    // nums.iter().map(|num| (num - base).abs()).sum()

    // 正确答案

    let mut subs = include_str!("../../../data/day7/input")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    // println!("{:?}", subs);

    let mid = subs.len() / 2;
    //思路：当med在移动的时候，只要其左边的数字个数比右边的少，那么结果一定是在减小的
    // 所以最优解一定落在中位数或者（数组长度为偶数）决定中位数的两个数构成的区间内
    let med = *subs.select_nth_unstable(mid).1; // 排序找中位数

    // println!("Got {}", med);

    subs.iter().map(|n| (n - med).abs()).sum::<i32>()
}

#[allow(dead_code)]
pub fn solve_day7_part2() -> i32 {
    let subs = include_str!("../../../data/day7/input")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();

    // 思路：考虑均值不等式 https://zh.m.wikipedia.org/zh-cn/%E5%B9%B3%E5%9D%87%E6%95%B0%E4%B8%8D%E7%AD%89%E5%BC%8F
    // 实际上，直接把结果关于x的函数列出来，然后求导令导数为0即可。又考虑到x只能取整数，所以就是均值的左右两个整数之一
    ((subs.iter().sum::<i32>() / subs.len() as i32) + 1..)
        .take(2)
        .map(|t| {
            subs.iter()
                .map(|n| {
                    let d = (n - t).abs();
                    d * (d + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}
