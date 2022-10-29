use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve_day10_part1() -> usize {
    let lines = include_bytes!("../../../data/day10/input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<_>>>();

    let mut pairs: HashMap<u8, u8> = HashMap::new();
    pairs.insert(b')', b'(');
    pairs.insert(b'}', b'{');
    pairs.insert(b']', b'[');
    pairs.insert(b'>', b'<');

    let mut cha = b'(';
    lines.iter().fold(0, |acc, x| {
        let mut st = vec![];
        let mut flag = true;
        for &c in x {
            if !flag {
                break;
            }
            match c {
                b'(' | b'{' | b'[' | b'<' => st.push(c),
                _ => {
                    if st.len() > 0 {
                        if st[st.len() - 1] != *(pairs.get(&c).unwrap()) {
                            flag = false;
                            cha = c;
                        } else {
                            st.pop();
                        }
                    }
                }
            }
        }
        match flag {
            true => acc,
            false => {
                // println!("acc: {}, cha: {}", acc, cha as char);
                acc + match cha {
                    b')' => 3,
                    b']' => 57,
                    b'}' => 1197,
                    _ => 25137,
                }
            }
        }
    })
}

#[allow(dead_code)]
pub fn solve_day10_part1_exp() -> usize {
    include_str!("../../../data/day10/input")
        .lines() // yield all lines
        .filter_map(|seq| {
            // find the corrupted lines
            seq.bytes()
                .scan(Vec::with_capacity(64), |s, c| {
                    Some(match c {
                        c if matches!(c, b'(' | b'[' | b'{' | b'<') => {
                            (s.push(c) != ()).then(|| b' ')
                        }
                        b')' => (s.pop().unwrap() != b'(').then(|| b')'),
                        c => (s.pop().unwrap() != c - 2).then(|| c),
                    })
                })
                .skip_while(Option::is_none) // find the first that is not None
                .map(|c| [3, 25137, 57, 1197][c.unwrap() as usize / 30 - 1])
                .next()
        })
        .sum::<usize>() // add them together
}

#[allow(dead_code)]
pub fn solve_day10_part2() -> usize {
    let mut scores = include_str!("../../../data/day10/input")
        .lines()
        .filter_map(|seq| {
            let mut stack = Vec::new();
            let mut flag = true;
            seq.bytes().for_each(|c| {
                match c {
                    b'(' | b'{' | b'[' | b'<' => stack.push(c),
                    b')' => {
                        if stack.len() == 0 || stack.pop().unwrap() != c - 1 {
                            flag = false;
                        }
                    }
                    _ => {
                        if stack.len() == 0 || stack.pop().unwrap() != c - 2 {
                            flag = false;
                        }
                    }
                }
                // println!(
                //     "After met {}, flag is {flag}, stack: {:?}",
                //     c as char,
                //     stack.iter().map(|&c| c as char).collect::<String>()
                // );
            });

            // println!("internal: {:?}, flag: {}", stack, flag);
            // then() returns Some(T) or None, which means Option<T>
            (flag && stack.len() > 0).then(|| {
                stack.iter().rev().fold(0, |acc, &c| {
                    (acc * 5)
                        + match c {
                            b'(' => 1,
                            b'[' => 2,
                            b'{' => 3,
                            _ => 4,
                        }
                })
            })
        })
        .collect::<Vec<usize>>();
    // println!("got: {:?}", scores);
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[allow(dead_code)]
pub fn solve_day10_part2_exp() -> usize {
    let mut scores = include_str!("../../../data/day10/input")
        .lines()
        .filter_map(|seq| {
            let mut s = Vec::with_capacity(64);
            seq.bytes()
                .all(|c| match c {
                    c if matches!(c, b'(' | b'[' | b'{' | b'<') => s.push(c) == (),
                    b')' => s.pop().unwrap() == b'(',
                    c => s.pop().unwrap() == c - 2,
                })
                .then(|| s)
        })
        .map(|s| {
            s.iter().rev().fold(0usize, |acc, &c| {
                acc * 5 + [1, 4, 2, 3][c as usize / 30 - 1]
            })
        })
        .collect::<Vec<_>>();

    let mid = scores.len() / 2;
    *(scores.select_nth_unstable(mid).1) // sort is unnecessary here.
}
