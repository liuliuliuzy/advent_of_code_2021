// const (MX, MY): (usize, usize) = (1310, 894);
const MX: usize = 1310;
const MY: usize = 894;

#[allow(dead_code)]
pub fn solve_day13_part1() -> usize {
    let (first, second) = include_str!("../../../data/day13/input")
        .split_once("\n\n")
        .unwrap();

    let mut dots = [[false; MY + 1]; MX + 1];
    first.split('\n').for_each(|l| {
        let (x, y) = l.split_once(',').unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        dots[x][y] = true;
    });

    let (dir, x) = second.split_once('\n').unwrap().0[11..]
        .split_once('=')
        .unwrap();
    let (dir, x) = (
        match dir {
            "x" => true,
            _ => false,
        },
        x.parse::<usize>().unwrap(),
    );
    // println!("{dir}, {x}");

    let (new_mx, new_my) = fold(&mut dots, MX, MY, dir, x);
    (0..=new_mx)
        .flat_map(|i| (0..=new_my).map(move |j| (i, j)))
        .filter(|&(i, j)| dots[i][j])
        .count()
}

#[allow(dead_code)]
fn fold(
    dots: &mut [[bool; MY + 1]; MX + 1],
    mx: usize,
    my: usize,
    dir: bool,
    x: usize,
) -> (usize, usize) {
    if dir {
        // fold left
        (0..=my).for_each(|j| dots[x][j] = false);
        (0..x).for_each(|i| {
            (0..=my).for_each(|j| {
                dots[i][j] |= dots[2 * x - i][j];
            })
        });
        (x - 1, my)
    } else {
        // fold up
        (0..=mx).for_each(|i| dots[i][x] = false);
        (0..=mx).for_each(|i| {
            (0..x).for_each(|j| {
                dots[i][j] |= dots[i][2 * x - j];
            })
        });
        (mx, x - 1)
    }
    // (MX, MY)
}

#[allow(dead_code)]
pub fn solve_day13_part2() -> usize {
    let (first, second) = include_str!("../../../data/day13/input")
        .split_once("\n\n")
        .unwrap();

    let mut dots = [[false; MY + 1]; MX + 1];
    first.split('\n').for_each(|l| {
        let (x, y) = l.split_once(',').unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        dots[x][y] = true;
    });

    let (new_mx, new_my) = second.split('\n').fold((MX, MY), |(mx, my), l| {
        let (dir, x) = l[11..].split_once('=').unwrap();
        let (dir, x) = (
            match dir {
                "x" => true,
                _ => false,
            },
            x.parse::<usize>().unwrap(),
        );
        fold(&mut dots, mx, my, dir, x)
    });

    // println!("{:?}", (new_mx, new_my));

    // println!("{dir}, {x}");
    for j in 0..=new_my {
        for i in 0..new_mx {
            print!("{}", if dots[i][j] { '█' } else { ' ' });
        }
        println!("");
    }
    // got: AHPRPAUZ

    (0..=new_mx)
        .flat_map(|i| (0..=new_my).map(move |j| (i, j)))
        .filter(|&(i, j)| dots[i][j])
        .count()
}
