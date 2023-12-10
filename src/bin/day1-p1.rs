fn main() {
    let input = include_str!("../../inputs/day1.input");

    let output = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter_map(|c| { c.to_digit(10) });
            let first = chars.next().expect("line to have at least 1 digit char");
            let last = chars.next_back().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>();

    println!("{output}");
}
