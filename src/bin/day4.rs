fn main() {
    let input = include_str!("../../input/day4.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn check_around(map: &[Vec<bool>], col: usize, row: usize) -> u32 {
    map[col - 1][row - 1] as u32
        + map[col - 1][row] as u32
        + map[col - 1][row + 1] as u32
        + map[col][row - 1] as u32
        + map[col][row + 1] as u32
        + map[col + 1][row - 1] as u32
        + map[col + 1][row] as u32
        + map[col + 1][row + 1] as u32
}

fn part1(input: &str) -> u32 {
    let mut map = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    map.iter_mut().for_each(|line| {
        line.insert(0, false);
        line.push(false);
    });

    map.insert(0, vec![false; map[0].len()]);
    map.push(vec![false; map[0].len()]);

    let mut total = 0;

    for col in 1..map.len() - 1 {
        for row in 1..map[0].len() - 1 {
            if map[col][row] && check_around(&map, col, row) < 4 {
                total += 1;
            }
        }
    }

    total
}

fn part2(input: &str) -> u32 {
    let mut map = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    map.iter_mut().for_each(|line| {
        line.insert(0, false);
        line.push(false);
    });

    map.insert(0, vec![false; map[0].len()]);
    map.push(vec![false; map[0].len()]);

    let mut total = 0;
    let mut found = true;

    while found {
        found = false;
        for col in 1..map.len() - 1 {
            for row in 1..map[0].len() - 1 {
                if map[col][row] && check_around(&map, col, row) < 4 {
                    map[col][row] = false;
                    total += 1;
                    found = true;
                }
            }
        }
    }

    total
}
