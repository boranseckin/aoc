#![feature(new_range_api)]
#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]

use std::collections::HashSet;
use std::ops::IntoBounds;
use std::ops::RangeBounds;
use std::range::RangeInclusive;

fn main() {
    let input = include_str!("../../inputs/day5.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u64 {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges
        .lines()
        .map(|line| {
            let (s, e) = line.split_once("-").unwrap();
            s.parse::<u64>().unwrap()..=e.parse::<u64>().unwrap()
        })
        .collect();

    ids.lines()
        .map(|line| {
            let id = line.parse::<u64>().unwrap();
            for range in &ranges {
                if range.contains(&id) {
                    return 1;
                }
            }

            0
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<_> = ranges
        .lines()
        .map(|line| {
            let (s, e) = line.split_once("-").unwrap();
            RangeInclusive::from(s.parse::<u64>().unwrap()..=e.parse::<u64>().unwrap())
        })
        .collect();

    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    loop {
        let mut reduced = false;

        for i in 0..ranges.len() - 1 {
            if ranges[i] == ranges[i + 1] {
                continue;
            }

            let inter = ranges[i].intersect(ranges[i + 1]);
            if !inter.is_empty() {
                ranges[i].start = ranges[i].start.min(ranges[i + 1].start);
                ranges[i].last = ranges[i].last.max(ranges[i + 1].last);
                ranges[i + 1] = ranges[i];

                reduced = true;
            }
        }

        if !reduced {
            break;
        }
    }

    let ranges: HashSet<_> = ranges.iter().collect();

    ranges
        .iter()
        // +1 to include the end bound
        .map(|r| r.last - r.start + 1)
        .sum::<u64>()
}
