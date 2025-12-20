use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day7.txt");

    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let start = lines.next().unwrap().find("S").unwrap();

    let mut beams = vec![start];
    let mut count = 0;

    for line in lines {
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .for_each(|split| {
                if let Some(i) = beams.iter().position(|&b| b == split) {
                    count += 1;
                    beams.remove(i);
                    if !beams.contains(&(split - 1)) {
                        beams.push(split - 1);
                    }
                    if !beams.contains(&(split + 1)) {
                        beams.push(split + 1);
                    }
                }
            });
    }

    count
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut memory = HashMap::<(usize, usize), u64>::new();

    let start = lines.next().unwrap().find("S").unwrap();

    let lines: Vec<_> = lines.collect();

    let mut count = 0;

    count += follow(0, start, &lines, &mut memory);

    count
}

fn follow(
    line: usize,
    index: usize,
    lines: &Vec<&str>,
    memory: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if lines.len() == line {
        return 1;
    }

    if let Some(mem) = memory.get(&(line, index)) {
        return *mem;
    }

    let next = lines[line].chars().nth(index).unwrap();

    let result = match next {
        '.' => follow(line + 1, index, lines, memory),
        '^' => {
            follow(line + 1, index - 1, lines, memory) + follow(line + 1, index + 1, lines, memory)
        }
        _ => unreachable!(),
    };

    memory.insert((line, index), result);

    result
}
