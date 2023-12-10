fn main() {
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
