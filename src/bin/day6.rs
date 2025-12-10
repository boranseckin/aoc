fn main() {
    let input = include_str!("../../input/day6.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut results: Vec<u64> = vec![];

    let ops: Vec<_> = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|c| match c {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!(),
        })
        .collect();

    ops.iter().for_each(|op| match op {
        Op::Add => results.push(0),
        Op::Mul => results.push(1),
    });

    for line in lines {
        line.split_whitespace()
            .map(|c| c.parse::<u64>().unwrap())
            .enumerate()
            .for_each(|(i, num)| match ops[i] {
                Op::Add => results[i] += num,
                Op::Mul => results[i] *= num,
            });
    }

    results.iter().sum()
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines().peekable();

    let mut results: Vec<u64> = vec![];

    let ops: Vec<_> = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|c| match c {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!(),
        })
        .collect();

    ops.iter().for_each(|op| match op {
        Op::Add => results.push(0),
        Op::Mul => results.push(1),
    });

    let chars = lines
        .clone() // unfortunate
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // find empty columns
    let empty_indices = (0..chars[0].len())
        .filter(|&i| chars.iter().all(|line| line[i] == ' '))
        .collect::<Vec<_>>();

    // split each row into columns using the empty indeces
    let nums: Vec<_> = lines
        .map(|line| {
            let mut split: Vec<Vec<_>> = vec![];
            let mut rest = line;
            for i in 0..empty_indices.len() {
                let mut index = empty_indices[i];
                if i > 0 {
                    index -= empty_indices[i - 1];
                }

                let (a, b) = rest.split_at(index);
                rest = b;

                split.push(a.chars().collect());
                if i == empty_indices.len() - 1 {
                    split.push(b.chars().collect());
                }
            }
            split
        })
        .collect();

    // transpose matrix to be column-wise
    let operations: Vec<Vec<Vec<_>>> = (0..nums[0].len())
        .map(|i| nums.iter().map(|n| n[i].clone()).collect())
        .collect();

    operations.iter().enumerate().for_each(|(op_i, op)| {
        // each operation has the same maximum amount of numbers
        (0..op[0].len())
            // group digits from each column
            .map(|i| op.iter().map(move |n| n[i]).collect::<Vec<_>>())
            // combine digits
            .map(|c| String::from_iter(c).trim().parse::<u64>())
            .for_each(|num| {
                if let Ok(num) = num {
                    match ops[op_i] {
                        Op::Add => results[op_i] += num,
                        Op::Mul => results[op_i] *= num,
                    }
                }
            });
    });

    results.iter().sum()
}
