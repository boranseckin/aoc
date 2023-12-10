use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    let mut max = a.max(b);
    let mut min = a.min(b);

    loop {
        let tmp = max % min;
        if tmp == 0 {
            return min;
        }
        max = min;
        min = tmp;
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = include_str!("../../inputs/day8.input");

    let (directions, map) = input.split_once("\n\n").unwrap();

    let mut hash_map = HashMap::new();
    map.lines().for_each(|line| {
        let (location, branches) = line.split_once(" = ").unwrap();
        let branches = branches[1..branches.len() - 1].split_once(", ").unwrap();
        hash_map.insert(location, branches);
    });

    let mut current_locations: Vec<&str> = hash_map.keys().cloned().filter(|k| k.ends_with('A')).collect();
    let mut steps = 0;

    let directions: Vec<char> = directions.chars().collect();

    let mut aggreate_steps = Vec::new();

    loop {
        steps += 1;

        let direction = directions.get((steps - 1) % directions.len()).unwrap();

        current_locations.iter_mut().filter(|l| !l.ends_with('Z')).for_each(|location| {
            *location = match direction {
                'R' => {
                    hash_map.get(location).unwrap().1
                },
                'L' => {
                    hash_map.get(location).unwrap().0
                },
                _ => unreachable!(),
            };

            if location.ends_with('Z') {
                aggreate_steps.push(steps);
            }
        });

        if current_locations.iter().all(|l| l.ends_with('Z')) {
            break;
        }
    }

    let total_steps = aggreate_steps.into_iter().reduce(lcm).unwrap();

    println!("{total_steps}");
}
