fn main() {
    let input = include_str!("../../input/day3.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut max = u32::MIN;

        for i in 0..line.len() {
            for j in (i + 1)..line.len() {
                let num = (line[i] * 10) + line[j];
                if num > max {
                    max = num;
                }
            }
        }

        sum += max;
    }

    sum
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut max = vec![];
        let mut start = 0;

        for _ in 0..12 {
            let max_digit = line
                .iter()
                .skip(start)
                .take(line.len() - start - (12 - max.len()) + 1)
                .max()
                .unwrap();

            let max_index = line
                .iter()
                .skip(start)
                .position(|x| x == max_digit)
                .unwrap();

            start += max_index;
            max.push(*max_digit);
            start += 1;
        }

        let max: u64 = max
            .iter()
            .rev()
            .enumerate()
            .map(|(i, e)| (*e as u64) * 10u64.pow(i as u32))
            .sum();

        sum += max;
    }

    sum
}

#[test]
fn test_part2() {
    assert_eq!(part2("987654321111111"), 987654321111);
    assert_eq!(part2("811111111111119"), 811111111119);
    assert_eq!(part2("234234234234278"), 434234234278);
    assert_eq!(part2("818181911112111"), 888911112111);
}
