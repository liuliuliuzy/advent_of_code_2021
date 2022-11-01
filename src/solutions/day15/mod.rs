use pathfinding::directed::dijkstra;

#[allow(dead_code)]
pub fn part1_inner(input: &str) -> u32 {
    let graph = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|l| l.iter().map(|&n| (n - b'0') as u32).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (m, n) = (graph.len(), graph[0].len());
    println!("{:?}", (m, n));
    let mut dp = vec![vec![0 as u32; n]; m];

    (1..m).for_each(|i| dp[i][0] = dp[i - 1][0] + graph[i][0]);
    (1..n).for_each(|j| dp[0][j] = dp[0][j - 1] + graph[0][j]);

    // println!("{dp:?}");

    (1..m).for_each(|i| {
        (1..n).for_each(|j| {
            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + graph[i][j];
        })
    });

    // println!("{dp:?}");

    dp[m - 1][n - 1]
}

#[allow(dead_code)]
pub fn solve_day15_part1() -> u32 {
    part1_inner(include_str!("../../../data/day15/input"))
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[allow(dead_code)]
pub fn solve_day15_part1_exp_inner(input: &str) -> u32 {
    let graph: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();

    let dest = (graph.len() as i32 - 1, graph[0].len() as i32 - 1);
    dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            DIRS.iter()
                .map(|(xx, yy)| {
                    graph
                        .get((x + xx) as usize)
                        .and_then(|r| r.get((y + yy) as usize))
                        .map(|c| ((x + xx, y + yy), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == dest,
    )
    .unwrap()
    .1
}

#[allow(dead_code)]
pub fn solve_day15_part1_exp() -> u32 {
    solve_day15_part1_exp_inner(include_str!("../../../data/day15/input"))
}

#[allow(dead_code)]
pub fn solve_day15_part2() -> u32 {
    solve_day15_part2_inner(include_str!("../../../data/day15/input"))
}

#[allow(dead_code)]
pub fn solve_day15_part2_inner(input: &str) -> u32 {
    let g: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let (m, n) = (g.len(), g[0].len());
    let mut graph: Vec<Vec<u8>> = vec![vec![0; n * 5]; m * 5];

    (0..5)
        .flat_map(|i| (0..5).map(move |j| (i, j))) // why do we need to take the ownership of i here?
        .for_each(|(i, j)| {
            for x in 0..m {
                for y in 0..n {
                    graph[i * m + x][j * n + y] = match g[x][y] + (i + j) as u8 {
                        n if n == 9 => 9,
                        n => n % 9,
                    };
                }
            }
        });

    let dest = (5 * m as i32 - 1, 5 * n as i32 - 1);
    dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            DIRS.iter()
                .map(|(xx, yy)| {
                    graph
                        .get((x + xx) as usize)
                        .and_then(|r| r.get((y + yy) as usize))
                        .map(|c| ((x + xx, y + yy), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == dest,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let data = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(40, solve_day15_part1_exp_inner(data));
    }

    #[test]
    fn test_part2() {
        let data = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(315, solve_day15_part2_inner(data));
    }
}
