fn main() {
    let input = include_str!("../../inputs/day6.input");

    let mut input = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        });
    let times = input.next().unwrap();
    let distances = input.next().unwrap();

    let wins: usize = times.into_iter().zip(distances).map(|(time, distance)| {
        (1..time)
            .filter(|t| (time - t) * t > distance)
            .collect::<Vec<usize>>()
            .len()
    })
    .reduce(|acc, e| acc * e)
    .unwrap();

    println!("{wins}");
}
