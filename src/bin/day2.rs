fn main() {
    let input = include_str!("../../inputs/day2.input");

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut possible_games = Vec::new();

    input.lines().for_each(|game| {
        let Some((game_id, data)) = game.split_once(':') else { unreachable!() };
        let game_id: usize = game_id.split_once(' ').unwrap().1.parse().unwrap();

        let mut is_possible = true;

        data.split(';').for_each(|round| {
            round.split(',').for_each(|color| {
                let mut a = color.split_whitespace();
                let num: usize = a.next().unwrap().parse().unwrap();
                let col = a.next().unwrap();

                match col {
                    "red" => { if num > max_red { is_possible = false } },
                    "green" => { if num > max_green { is_possible = false } },
                    "blue" => { if num > max_blue { is_possible = false } },
                    _ => unreachable!(),
                }
            });
        });

        if is_possible {
            possible_games.push(game_id);
        }
    });

    println!("{}", possible_games.into_iter().sum::<usize>());


    let input = include_str!("../../inputs/day2.input");

    let mut powers = Vec::new();

    input.lines().for_each(|game| {
        let Some((_, data)) = game.split_once(':') else { unreachable!() };

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        data.split(';').for_each(|round| {
            round.split(',').for_each(|color| {
                let mut a = color.split_whitespace();
                let num: usize = a.next().unwrap().parse().unwrap();
                let col = a.next().unwrap();

                match col {
                    "red" => { if num > red { red = num } },
                    "green" => { if num > green { green = num } },
                    "blue" => { if num > blue { blue = num } },
                    _ => unreachable!(),
                }
            });
        });

        powers.push(red * green * blue);
    });

    println!("{}", powers.into_iter().sum::<usize>());
}
