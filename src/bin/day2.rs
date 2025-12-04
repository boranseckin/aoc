use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day2.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.trim_end().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for i in start..=end {
            let s = i.to_string();
            let len = s.len();
            if len % 2 == 0 {
                let (left, right) = s.split_at(len / 2);
                if left == right {
                    sum += i;
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.trim_end().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        for i in start..=end {
            let s = i.to_string();
            let len = s.len();

            let chars = s.chars().collect::<Vec<char>>();

            for j in 1..=(len / 2) {
                if len % 2 != 0 && j % 2 == 0 {
                    continue;
                }

                if chars.chunks(j).all_equal() {
                    sum += i;
                    break;
                }
            }
        }
    }

    sum
}
