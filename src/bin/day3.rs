use std::collections::HashSet;

fn find_adjacent_nums(line: &Vec<char>, char_no: usize) -> (usize, usize) {
    let end: Vec<&char> = line
        .iter()
        .skip(char_no)
        .take_while(|c| c.is_ascii_digit())
        .collect();

    let mut start: Vec<&char> = line
        .iter()
        .rev()
        .skip(line.len() - char_no)
        .take_while(|c| c.is_ascii_digit())
        .collect::<Vec<&char>>()
        .into_iter()
        .rev()
        .collect();

    // starting position of the number
    let position = char_no - start.len();

    start.extend(end);
    let number = start
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap();

    (position, number)
}

fn main() {
    let input = include_str!("../../inputs/day3.input");

    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut numbers = HashSet::new();

    let matrix = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for (line_no, line) in lines.iter().enumerate() {
        for (char_no, char) in line.iter().enumerate() {
            if char != &'.' && char.is_ascii_punctuation() {
                for location in matrix {
                    if let Some(l) = lines.get((line_no as i32 + location.0) as usize) {
                        if let Some(c) = l.get((char_no as i32 + location.1) as usize) {
                            if c.is_ascii_digit() {
                                let (position, number) = find_adjacent_nums(
                                    l,
                                    (char_no as i32 + location.1) as usize
                                );
                                numbers.insert((line_no, position, number));
                            }
                        }
                    }
                }
            }
        }
    }

    let sum: usize = numbers.into_iter().map(|num| num.2).sum();
    dbg!(sum);


    let input = include_str!("../../inputs/day3.input");

    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let matrix = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut ratios = Vec::new();

    for (line_no, line) in lines.iter().enumerate() {
        for (char_no, char) in line.iter().enumerate() {
            if char == &'*' {
                let mut numbers = HashSet::new();

                for location in matrix {
                    if let Some(l) = lines.get((line_no as i32 + location.0) as usize) {
                        if let Some(c) = l.get((char_no as i32 + location.1) as usize) {
                            if c.is_ascii_digit() {
                                let (position, number) = find_adjacent_nums(
                                    l,
                                    (char_no as i32 + location.1) as usize
                                );
                                numbers.insert((line_no, position, number));
                            }
                        }
                    }
                }

                if numbers.len() == 2 {
                    ratios.push(
                        numbers.
                        into_iter()
                        .map(|n| n.2)
                        .reduce(|acc, n| acc * n)
                        .unwrap()
                    );
                }
            }
        }
    }

    let sum: usize = ratios.into_iter().sum();
    dbg!(sum);
}
