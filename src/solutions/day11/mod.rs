const SIZE: usize = 10;
const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

#[allow(dead_code)]
pub fn solve_day11_part1() -> usize {
    let mut g = include_bytes!("../../../data/day11/input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<_>>>();

    // count 100 steps
    (0..100).fold(0, |acc, _| {
        // every cell's energy increases by 1
        g.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));
        acc + (0..SIZE)
            .flat_map(|x| (0..SIZE).map(move |y| (x, y)))
            .fold(0, |acc, (x, y)| {
                acc + (g[x][y] > b'9').then(|| flash(&mut g, x, y)).unwrap_or(0)
            })
    })
}

#[allow(dead_code)]
fn flash(g: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    // dfs
    g[x][y] = b'0'; // 0 means it just flashed
    DIRS.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match g.get_mut(x).and_then(|l| l.get_mut(y)) {
                Some(cell) if *cell > b'0' => {
                    *cell += 1;
                    acc + (*cell > b'9').then(|| flash(g, x, y)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}

#[allow(dead_code)]
pub fn solve_day11_part2() -> usize {
    let mut g = include_bytes!("../../../data/day11/input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<_>>>();

    let mut ans = 0;
    // (0..300).for_each(|_| {
    loop {
        ans += 1;
        g.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));

        (0..SIZE)
            .flat_map(|x| (0..SIZE).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                if g[x][y] > b'9' {
                    flash(&mut g, x, y);
                }
            });

        // println!("round {ans}");
        // g.iter().for_each(|row| {
        //     row.iter().for_each(|cell| print!("{} ", *cell as char));
        //     println!("");
        // });

        if g.iter().all(|row| row.iter().all(|cell| *cell == b'0')) {
            break;
        }
    }
    ans

    // ================
    // example solution
    // ================
    // https://github.com/timvisee/advent-of-code-2021/blob/master/day11b/src/main.rs
    // (1..)
    //     .find(|_| {
    //         g.iter_mut()
    //             .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));

    //         // count the flash times
    //         (0..SIZE)
    //             .flat_map(|x| (0..SIZE).map(move |y| (x, y)))
    //             .fold(0, |acc, (x, y)| {
    //                 acc + (g[x][y] > b'9').then(|| flash(&mut g, x, y)).unwrap_or(0)
    //             })
    //             == SIZE * SIZE
    //     })
    //     .unwrap()
}
