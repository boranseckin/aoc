fn map_range(num: usize, ranges: &[Vec<usize>]) -> usize {
    for range in ranges {
        if num >= range[1] && num < range[1] + range[2] {
            return range[0] + (num - range[1]);
        }
    }
    num
}

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

    println!("{min_location}");
}
