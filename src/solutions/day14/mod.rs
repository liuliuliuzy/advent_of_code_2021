// use std::collections::HashMap;

use std::mem;

#[allow(dead_code)]
pub fn solve_day14_part1() -> usize {
    let (first, second) = include_str!("../../../data/day14/input")
        .split_once("\n\n")
        .unwrap();

    let base = first.as_bytes().to_vec();

    let mut rules = second
        .lines()
        .map(|l| {
            let (k, v) = l.split_once(" -> ").unwrap();
            let (k, v) = (k.as_bytes(), v.as_bytes()[0]);
            ([k[0], k[1]], [k[0], v])
        })
        .collect::<Vec<_>>();
    rules.sort_unstable_by_key(|r| r.0);
    // println!(
    //     "rules: {:?}",
    //     rules
    //         .iter()
    //         .map(|r| (
    //             [r.0[0] as char, r.0[1] as char],
    //             [r.1[0] as char, r.1[1] as char]
    //         ))
    //         .collect::<Vec<_>>()
    // );

    let rules = rules
        .iter()
        .map(|r| {
            (
                r.0,
                rules.binary_search_by_key(&r.1, |r| r.0).unwrap(),
                rules
                    .binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0)
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    // println!("rules: {rules:?}");

    let (mut num, mut next) = (vec![0; rules.len()], vec![0; rules.len()]);
    base.windows(2)
        .for_each(|key| num[rules.binary_search_by_key(&key, |r| &r.0).unwrap()] += 1);
    (0..10).for_each(|_| {
        num.iter_mut().zip(&rules).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        mem::swap(&mut num, &mut next);
    });

    let mut occur = [0; (b'Z' - b'A') as usize];
    occur[(base.last().unwrap() - b'A') as usize] += 1;
    rules
        .iter()
        .zip(num)
        .for_each(|(r, n)| occur[(r.0[0] - b'A') as usize] += n);

    let (min, max) = occur
        .iter()
        .filter(|&&n| n != 0)
        .fold((usize::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)));

    max - min
}

#[allow(dead_code)]
pub fn solve_day14_part2() -> u64 {
    let (first, second) = include_str!("../../../data/day14/input")
        .split_once("\n\n")
        .unwrap();

    let base = first.as_bytes().to_vec();

    let mut rules = second
        .lines()
        .map(|l| {
            let (k, v) = l.split_once(" -> ").unwrap();
            let (k, v) = (k.as_bytes(), v.as_bytes()[0]);
            ([k[0], k[1]], [k[0], v])
        })
        .collect::<Vec<_>>();
    rules.sort_unstable_by_key(|r| r.0);

    let rules = rules
        .iter()
        .map(|r| {
            (
                r.0,
                rules.binary_search_by_key(&r.1, |r| r.0).unwrap(),
                rules
                    .binary_search_by_key(&[r.1[1], r.0[1]], |r| r.0)
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    // println!("rules: {rules:?}");

    let (mut num, mut next) = (vec![0; rules.len()], vec![0; rules.len()]);
    base.windows(2)
        .for_each(|key| num[rules.binary_search_by_key(&key, |r| &r.0).unwrap()] += 1);
    (0..40).for_each(|_| {
        num.iter_mut().zip(&rules).for_each(|(n, r)| {
            next[r.1] += *n;
            next[r.2] += *n;
            *n = 0;
        });
        mem::swap(&mut num, &mut next);
    });

    let mut occur = [0; (b'Z' - b'A') as usize];
    occur[(base.last().unwrap() - b'A') as usize] += 1;
    rules
        .iter()
        .zip(num)
        .for_each(|(r, n)| occur[(r.0[0] - b'A') as usize] += n);

    let (min, max) = occur
        .iter()
        .filter(|&&n| n != 0)
        .fold((u64::MAX, 0), |(min, max), &n| (min.min(n), max.max(n)));

    max - min
}
