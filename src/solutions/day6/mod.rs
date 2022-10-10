#[allow(dead_code)]
pub fn solve_day6_part1() -> i32 {
    // 数学计算较为复杂，所以这里直接模拟
    // 统计每种天数对应的laternfish数量
    let mut mp = include_str!("../../../data/day6/input")
        .split(',')
        .fold([0; 9], |mut mp, d| {
            // println!("{}", d);
            mp[d.parse::<usize>().unwrap()] += 1;
            mp
        });
    // println!("Initial: {:?}", mp);

    for _ in 1..=80 {
        let (count_6, count_8) = (mp[0], mp[0]);
        for i in 0..8 {
            mp[i] = mp[i + 1];
        }
        mp[6] += count_6;
        mp[8] = count_8;
        // println!("After:  {:?}", mp);
    }

    // 下面的操作也是正确的，但是不太好理解
    // (1..80).for_each(|day| {
    //     mp[(day + 7) % 9] += mp[day % 9];
    //     println!("After:   {:?}", mp);
    // });

    mp.iter().sum()
}

#[allow(dead_code)]
pub fn solve_day6_part2() -> u64 {
    // 数学计算较为复杂，所以这里直接模拟
    // 统计每种天数对应的laternfish数量
    let mut mp = include_str!("../../../data/day6/input")
        .split(',')
        .fold([0; 9], |mut mp, d| {
            // println!("{}", d);
            mp[d.parse::<usize>().unwrap()] += 1;
            mp
        });
    // println!("Initial: {:?}", mp);

    for _ in 1..=256 {
        let (count_6, count_8) = (mp[0], mp[0]);
        for i in 0..8 {
            mp[i] = mp[i + 1];
        }
        mp[6] += count_6;
        mp[8] = count_8;
        // println!("After:  {:?}", mp);
    }

    // 下面的操作也是正确的，但是不太好理解
    // (1..80).for_each(|day| {
    //     mp[(day + 7) % 9] += mp[day % 9];
    //     println!("After:   {:?}", mp);
    // });

    mp.iter().sum()
}
