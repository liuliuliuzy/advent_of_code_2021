// Refer to https://github.com/timvisee/advent-of-code-2021/blob/master/day12a/src/main.rs
// And added some comments by myself :-)
use std::collections::BTreeMap;

const EDGES: usize = 12;

#[allow(dead_code)]
pub fn solve_day12_part1() -> usize {
    // it seems like a puzzle about graph algorithm

    let mut uc = vec![];

    // For the difference between HashMap & BTreeMap, see: https://www.reddit.com/r/rust/comments/7rgowj/hashmap_vs_btreemap/

    // store
    let mut id: BTreeMap<&str, u8> = BTreeMap::from([("start", 0), ("end", 1)]);
    // store all the neighbors of every node
    let mut graph: BTreeMap<u8, Vec<u8>> = BTreeMap::new();

    // construct the graph
    include_str!("../../../data/day12/input")
        // include_str!("../../../data/day12/test")
        .lines()
        .for_each(|line| {
            // calculate the index of a node
            let mut idx = |a| {
                let next_id = id.len() as u8;
                // if not exist, then insert new value into it
                *id.entry(a).or_insert_with(|| {
                    // if the first letter of a is uppercase, add it to uc
                    (a.as_bytes()[0].is_ascii_uppercase()).then(|| uc.push(next_id));
                    next_id
                })
            };

            // add edge: a->b
            let mut branch = |a, b| {
                let entry = graph.entry(a).or_insert_with(|| Vec::with_capacity(6));
                (b != 0).then(|| entry.push(b));
            };

            // get two nodes
            let (a, b) = line.split_once('-').unwrap();
            let (a, b) = (idx(a), idx(b));

            // add two edges to graph
            branch(a, b);
            branch(b, a);
        });

    println!("id: {id:?}");
    println!("uc: {uc:?}");
    println!("{graph:?}");

    let graph = graph
        .keys()
        .filter(|&b| !uc.contains(b)) // filter lowercase node
        .map(|&b| {
            (
                b,
                graph[&b].iter().fold([0; EDGES], |mut children, child| {
                    if uc.contains(child) {
                        // Uppercase
                        graph[child].iter().for_each(|b| children[*b as usize] += 1);
                    } else {
                        // Lowercase
                        children[*child as usize] += 1;
                    }
                    children
                }),
            )
        })
        .collect::<BTreeMap<_, _>>();

    // graph[&a][b] = c means there are c different ways from a to b
    println!("{graph:?}");

    let mut stack: Vec<(u8, u8, usize)> = vec![(0, 1, 1)];
    let (mut to, mut count) = ([1; EDGES], 0);
    while let Some((a, b, s)) = stack.pop() {
        to[b as usize] = a;
        println!("graph[{a}]: {:?}", graph[&a]);
        count += graph[&a]
            .iter()
            .enumerate()
            .filter(|&(_, routes)| {
                // filter the neighbors that can be arrived
                // println!("{i}={:?}", *routes);
                *routes > 0
            })
            .fold(0, |acc, (node, routes)| match node {
                1 => acc + s * routes, // reach the end node, so add the count to the final result
                v => {
                    let visit = v != 0 && to[2..=b as usize].contains(&(v as u8));
                    (!visit).then(|| stack.push((v as u8, b + 1, s * graph[&a][v])));
                    acc
                }
            });
        println!("stack: {stack:?}");
        println!("to: {to:?}");
        println!("count: {count}");
    }

    count
}

#[allow(dead_code)]
pub fn solve_day12_part2() -> usize {
    let mut uc = vec![];
    let mut id: BTreeMap<&str, u8> = BTreeMap::from([("start", 1), ("end", 0)]);
    let mut map: BTreeMap<u8, Vec<u8>> = BTreeMap::new();

    include_str!("../../../data/day12/input")
        .lines()
        .for_each(|l| {
            let mut idx = |a| {
                let idx = id.len() as u8;
                *id.entry(a).or_insert_with(|| {
                    (a.as_bytes()[0] <= b'Z').then(|| uc.push(idx));
                    idx
                })
            };
            let mut branch = |a, b| {
                let entry = map.entry(a).or_insert_with(|| Vec::with_capacity(6));
                (b != 0).then(|| entry.push(b));
            };
            let (a, b) = l.split_once('-').unwrap();
            let (a, b) = (idx(a), idx(b));
            branch(a, b);
            branch(b, a);
        });

    let map = map
        .keys()
        .filter(|b| !uc.contains(b))
        .map(|&b| {
            (
                b,
                map[&b].iter().fold([0; EDGES], |mut chld, b| {
                    if uc.contains(b) {
                        map[b].iter().for_each(|b| chld[*b as usize] += 1);
                    } else {
                        chld[*b as usize] += 1;
                    }
                    chld
                }),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut todo: Vec<(u8, u8, bool, usize)> = vec![(0, 1, true, 1)];
    let (mut to, mut count) = ([1; EDGES], 0);
    while let Some((a, b, t, s)) = todo.pop() {
        to[b as usize] = a;
        count += map[&a]
            .iter()
            .enumerate()
            .filter(|&(_, routes)| *routes > 0)
            .fold(0, |acc, (c, _)| match c {
                1 => acc + s * map[&a][c],
                v => {
                    let visit = v != 0 && to[2..=b as usize].contains(&(v as u8));
                    (t || !visit).then(|| todo.push((v as u8, b + 1, t && !visit, s * map[&a][v])));
                    acc
                }
            });
    }

    count
}
