// use std::fs::File;

use std::collections::HashMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

#[allow(dead_code)]
pub fn solve_day4_part1() -> u32 {
    let (nums, boards) = include_str!("../../../data/day4/input")
        .split_once("\n\n")
        .unwrap();

    // println!("Got nums: {:?}", nums);

    let mut boards: Vec<(HashMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = nums
        .split(',')
        .map(|n| n.parse().unwrap()) // 这里应该是根据哈希表的<u8, usize>而自动推出parse的目标类型的
        .find_map(|n| {
            // findmap相当于根据迭代器进行遍历，在第一个满足条件的地方停下
            boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW)) // 判断是否存在第i列/第i行满足条件
                    .map(|_| (b.clone(), *m, n))
            })
        })
        .unwrap();

    board
        .into_iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * num as u32)
        .sum::<u32>()
}

#[allow(dead_code)]
pub fn solve_day4_part2() -> u32 {
    let (nums, boards) = include_str!("../../../data/day4/input")
        .split_once("\n\n")
        .unwrap();

    let mut boards: Vec<(HashMap<u8, usize>, u32)> = boards
        .split("\n\n")
        .map(|b| {
            (
                b.split_ascii_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (n, mut cnt) = (boards.len(), 0);
    let mut if_filled = vec![0; n];
    let (mut last_board, mut last_num) = (0, 0);
    for num in nums.split(',').map(|n| n.parse().unwrap()) {
        // 遍历每个棋盘
        for (board_idx, (board, m)) in boards.iter_mut().enumerate() {
            if if_filled[board_idx] == 1 {
                continue;
            }
            // 如果棋盘中存在这个数字
            if let Some(i) = board.get(&num) {
                *m |= 1 << *i; // 标记其位置
                               // 判断是否满足条件
                if (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW) {
                    cnt += 1;
                    if_filled[board_idx] = 1;
                    // 记录下上一个的情况
                    last_board = board_idx;
                    last_num = num;
                    // println!("filed one with: {}", num);
                    if cnt == n {
                        // println!("got number: {}", num);
                        // 找到了最后一个，返回结果
                        return board
                            .into_iter()
                            .map(|(n1, i)| (*m >> *i & 1 ^ 1) * *n1 as u32 * num as u32)
                            .sum::<u32>();
                    }
                }
            }
        }
    }

    let (target_board, mark) = boards[last_board].clone();
    target_board
        .into_iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * last_num as u32)
        .sum::<u32>()
}
