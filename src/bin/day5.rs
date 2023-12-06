fn main() {
    let input = include_str!("../../inputs/day5.input");

    let mut section = input.split("\n\n");

    let seeds: Vec<usize> = section
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|n| n.parse())
        .collect();

    let maps: Vec<Vec<Vec<usize>>> = section.map(|map| {
        map.lines().skip(1).map(|entry| {
            entry.split_whitespace().flat_map(|n| n.parse()).collect()
        }).collect()
    }).collect();

    let min_location: usize = seeds.into_iter().map(|mut seed| {
        for map in &maps {
            seed = map_range(seed, map);
        }
        seed
    }).min().unwrap();

    dbg!(min_location);


    let input = include_str!("../../inputs/day5.input");

    let mut section = input.split("\n\n");

    let mut seed_ranges = section
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap());

    let mut seeds = Vec::new();

    while let Some(start) = seed_ranges.next() {
        if let Some(range) = seed_ranges.next() {
            dbg!(start, range);
            seeds.reserve(range);
            (start..start + range).for_each(|seed| {
                seeds.push(seed);
            });
        }
    }

    let maps: Vec<Vec<Vec<usize>>> = section.map(|map| {
        map.lines().skip(1).map(|entry| {
            entry.split_whitespace().flat_map(|n| n.parse()).collect()
        }).collect()
    }).collect();

    let length = seeds.len();
    let min_location: usize = seeds.into_iter().enumerate().map(|(i, mut seed)| {
        if i % 100000 == 0 {
            println!("{i}/{length} %{}", i/length*100);
        }
        for map in &maps {
            seed = map_range(seed, map);
        }
        seed
    }).min().unwrap();

    dbg!(min_location);
}

fn map_range(num: usize, ranges: &[Vec<usize>]) -> usize {
    for range in ranges {
        if num >= range[1] && num <= range[1] + range[2] {
            return range[0] + (num - range[1]);
        }
    }
    num
}
