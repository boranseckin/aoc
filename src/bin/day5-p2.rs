use std::thread;
use std::sync::Arc;
use std::sync::mpsc;

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
    let length = seeds.len();

    let maps: Vec<Vec<Vec<usize>>> = section.map(|map| {
        map.lines().skip(1).map(|entry| {
            entry.split_whitespace().flat_map(|n| n.parse()).collect()
        }).collect()
    }).collect();
    let maps = &(*Box::leak(Box::new(maps)));

    let (tx, rx) = mpsc::channel();
    let seeds = Arc::new(seeds);

    let worker_count = 16;
    let chunk_len = ((length as f32 / worker_count as f32).round() as usize).max(1);
    dbg!(seeds.chunks(chunk_len).count());

    for i in 0..seeds.chunks(chunk_len).count() {
        let _tx = tx.clone();
        let _seeds = Arc::clone(&seeds);

        thread::spawn(move || {
            let seeds = _seeds.chunks(chunk_len).nth(i).unwrap();
            let min_location: usize = seeds.iter().map(|seed| {
                let mut seed = *seed;
                for map in maps {
                    seed = map_range(seed, map);
                }
                seed
            }).min().unwrap();

            _tx.send(min_location).unwrap();
        });
    }

    drop(tx);

    let mut results = Vec::with_capacity(worker_count);
    for recieved in rx {
        results.push(recieved);
    }

    println!("{}", results.into_iter().min().unwrap());
}
