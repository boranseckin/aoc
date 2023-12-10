fn main() {
    let input = include_str!("../../inputs/day6.input");

    let mut input = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        });
    let time = input.next().unwrap();
    let distance = input.next().unwrap();

    let wins = (1..time)
        .filter(|t| (time - t) * t > distance)
        .collect::<Vec<usize>>()
        .len();

    println!("{wins}");
}
