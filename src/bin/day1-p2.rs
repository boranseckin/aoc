fn main() {
    let input = include_str!("../../inputs/day1.input");

    let output = input
        .lines()
        .map(|line| {
            // position, value
            let mut nums = Vec::new();

            let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            for (index, digit) in digits.into_iter().enumerate() {
                for (pos, _) in line.match_indices(digit) {
                    nums.push((pos, index))
                }
            }

            let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
            for (index, word) in words.into_iter().enumerate() {
                for (pos, _) in line.match_indices(word) {
                    nums.push((pos, index + 1));
                }
            }

            nums.sort_by_key(|n| n.0);

            let first = nums.first().expect("line to have at least 1 number");
            let last = nums.last().unwrap_or(first);
            first.1 * 10 + last.1
        })
        .sum::<usize>();

    println!("{output}");
}
