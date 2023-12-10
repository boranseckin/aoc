fn calculate(tally: &mut Vec<usize>, copies: &Vec<Vec<usize>>, card_no: usize) {
    tally[card_no] += 1;

    if !copies[card_no].is_empty() {
        copies[card_no].iter().for_each(|copy| {
            calculate(tally, copies, copy - 1);
        });
    }
}

fn main() {
    let input = include_str!("../../inputs/day4.input");

    let copies = input.lines().map(|line| {
        let (card_no, line) = line.split_once(": ").unwrap();
        let (win_nums, nums) = line.split_once(" | ").unwrap();

        let card_no: usize = card_no.split_whitespace().nth(1).unwrap().parse().unwrap();

        let win_nums: Vec<usize> = win_nums.split_whitespace().flat_map(|num| num.parse()).collect();
        let mut nums: Vec<usize> = nums.split_whitespace().flat_map(|num| num.parse()).collect();

        nums.retain(|num| win_nums.contains(num));

        (card_no + 1..=card_no + nums.len()).collect()
    }).collect::<Vec<Vec<usize>>>();

    let mut tally = vec![0; copies.len()];

    (0..copies.len()).for_each(|card_no| {
        calculate(&mut tally, &copies, card_no);
    });

    let result: usize = tally.into_iter().sum();
    println!("{result}");
}
