use itertools::Itertools;

fn get_next(numbers: &[i32]) -> i32 {
    let diffs: Vec<i32> = numbers
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();

    if diffs.iter().all_equal() {
        *diffs.first().unwrap()
    } else {
        *diffs.first().unwrap() - get_next(&diffs)
    }
}

fn main() {
    let input = include_str!("../../inputs/day9.input");

    let next_nums = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .map(|history| {
            history.first().unwrap() - get_next(&history)
        })
        .collect::<Vec<_>>();

    println!("{}", next_nums.into_iter().sum::<i32>());
}
