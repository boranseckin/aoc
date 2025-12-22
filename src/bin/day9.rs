fn main() {
    let input = include_str!("../../input/day9.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u64 {
    let red_tiles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let line = line.split_once(',').unwrap();
            (line.1.parse().unwrap(), line.0.parse().unwrap())
        })
        .collect();

    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            // +1 for distance since 0-indexed
            let y = red_tiles[i].0.abs_diff(red_tiles[j].0) + 1;
            let x = red_tiles[i].1.abs_diff(red_tiles[j].1) + 1;

            let area = y * x;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as u64
}

fn part2(input: &str) -> u64 {
    let mut max_size = (0, 0);

    let mut red_tiles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let line = line.split_once(',').unwrap();
            let line = (line.1.parse().unwrap(), line.0.parse().unwrap());

            max_size = (max_size.0.max(line.0), max_size.1.max(line.1));

            line
        })
        .collect();

    red_tiles.push(red_tiles[0]);

    let mut safe = vec![(usize::MAX, usize::MIN); max_size.0 + 1];

    for i in 1..red_tiles.len() {
        let r1 = red_tiles[i - 1].0;
        let c1 = red_tiles[i - 1].1;

        let r2 = red_tiles[i].0;
        let c2 = red_tiles[i].1;

        if r1 == r2 {
            // rows are the same, columns are updated
            let c_min = c1.min(c2);
            let c_max = c1.max(c2);

            safe[r1] = (safe[r1].0.min(c_min), safe[r1].1.max(c_max));
        } else if c1 == c2 {
            // columns are the same, rows are updated
            let r_min = r1.min(r2);
            let r_max = r1.max(r2);

            for i in r_min..=r_max {
                safe[i] = (safe[i].0.min(c1), safe[i].1.max(c1));
            }
        }
    }

    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let r_min = red_tiles[i].0.min(red_tiles[j].0);
            let r_max = red_tiles[i].0.max(red_tiles[j].0);
            let c_min = red_tiles[i].1.min(red_tiles[j].1);
            let c_max = red_tiles[i].1.max(red_tiles[j].1);

            let mut safe_zone = true;
            for r in r_min..=r_max {
                if c_min < safe[r].0 || c_max > safe[r].1 {
                    safe_zone = false;
                    break;
                }
            }

            if !safe_zone {
                continue;
            }

            // +1 for distance since 0-indexed
            let y = red_tiles[i].0.abs_diff(red_tiles[j].0) + 1;
            let x = red_tiles[i].1.abs_diff(red_tiles[j].1) + 1;

            let area = y * x;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area as u64
}
