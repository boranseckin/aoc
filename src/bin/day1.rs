fn main() {
    let input = include_str!("../../input/day1.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u32 {
    let mut current: i32 = 50;
    let mut zero = 0;

    for line in input.lines() {
        let (dir, step) = line.split_at(1);
        let step = step.parse::<i32>().unwrap();
        let is_right = dir == "R";

        if is_right {
            current += step;
            current %= 100;
        } else {
            current -= step;
            current %= 100;
            if current < 0 {
                current += 100
            }
        }

        zero += if current == 0 { 1 } else { 0 };
    }

    zero
}

fn part2(input: &str) -> u32 {
    let mut current: i32 = 50;
    let mut zero = 0;

    for line in input.lines() {
        let (dir, step) = line.split_at(1);
        let step = step.parse::<i32>().unwrap();
        let is_right = dir == "R";

        if is_right {
            current += step;
            if current > 99 {
                let nrotation = current / 100;
                zero += nrotation;

                current %= 100;
            }
        } else {
            let starting_at_zero = current == 0;
            current -= step;
            if current <= 0 {
                current = current.abs();

                // +1 since first pass over 0 is <100
                let nrotation = (current / 100) + if starting_at_zero { 0 } else { 1 };
                zero += nrotation;

                current %= 100;
                current = if current == 0 { 0 } else { 100 - current };
            }
        }
    }

    zero.try_into().unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2("L50"), 1);
    assert_eq!(part2("L51"), 1);
    assert_eq!(part2("L49"), 0);
    assert_eq!(part2("R50"), 1);
    assert_eq!(part2("R51"), 1);
    assert_eq!(part2("R49"), 0);
    assert_eq!(part2("R149"), 1);
    assert_eq!(part2("R150"), 2);
    assert_eq!(part2("R151"), 2);
    assert_eq!(part2("L149"), 1);
    assert_eq!(part2("L150"), 2);
    assert_eq!(part2("L151"), 2);
    assert_eq!(part2("R1000"), 10);
    assert_eq!(part2("L1000"), 10);
    assert_eq!(part2("L50\nR2"), 1);
    assert_eq!(part2("L50\nL2"), 1);
}
