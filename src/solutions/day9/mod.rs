#[allow(dead_code)]
pub fn solve_day9_part1() -> usize {
    let map = include_bytes!("../../../data/day9/input")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();
    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut res = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if neighbors.iter().all(|&(xx, yy)| {
                map.get((i as isize + xx) as usize)
                    .and_then(|l| l.get((j as isize + yy) as usize))
                    .map(|n| cell < n)
                    .unwrap_or(true)
            }) {
                res += (cell - b'0') as usize + 1;
            }
        }
    }
    res
}

const NEXT: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[allow(dead_code)]
pub fn solve_day9_part2() -> usize {
    // bfs in rust?
    let mut g = include_bytes!("../../../data/day9/input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec()) // slice.to_vec
        .collect::<Vec<_>>();

    let mut basins = vec![];
    for x in 0..g.len() {
        for y in 0..g[0].len() {
            (g[x][y] < b'9').then(|| basins.push(basin(&mut g, x, y)));
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product::<usize>()
}

// do bfs
fn basin(g: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    g[x][y] = b'9'; // mark as visited
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match g.get(x).and_then(|l| l.get(y)).map(|&n| n < b'9') {
                Some(true) => acc + basin(g, x, y),
                _ => acc,
            }
        }) // iterate four directions
}
