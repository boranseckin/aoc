use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day8.input");

    let (directions, map) = input.split_once("\n\n").unwrap();

    let mut hash_map = HashMap::new();
    map.lines().for_each(|line| {
        let (location, branches) = line.split_once(" = ").unwrap();
        let branches = branches[1..branches.len() - 1].split_once(", ").unwrap();
        hash_map.insert(location, branches);
    });

    let mut current_location = "AAA";
    let mut steps = 0;

    while current_location != "ZZZ" {
        for direction in directions.chars() {
            steps += 1;

            match direction {
                'R' => {
                    current_location = hash_map.get(current_location).unwrap().1;
                },
                'L' => {
                    current_location = hash_map.get(current_location).unwrap().0;
                },
                _ => unreachable!(),
            };
        }
    }

    println!("{steps}");
}
