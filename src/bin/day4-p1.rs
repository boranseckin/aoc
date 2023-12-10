fn main() {
    let input = include_str!("../../inputs/day4.input");

    let sum: usize = input.lines().map(|line| {
        let (_card_no, line) = line.split_once(": ").unwrap();
        let (win_nums, nums) = line.split_once(" | ").unwrap();

        let win_nums: Vec<usize> = win_nums.split_whitespace().flat_map(|num| num.parse()).collect();
        let mut nums: Vec<usize> = nums.split_whitespace().flat_map(|num| num.parse()).collect();

        nums.retain(|num| win_nums.contains(num));

        if nums.is_empty() {
            0
        } else {
            2_usize.pow(nums.len() as u32 - 1)
        }
    }).sum();

    println!("{sum}");
}
