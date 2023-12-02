use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Day 2 Part 2");
    // open and parse file
    let mut file = File::open("src/input_2").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", get_part2_anwser(&contents));
}

fn get_part2_anwser(contents: &str) -> u32 {
    let mut t = 0;
    let _ = contents
        .lines()
        .enumerate()
        .for_each(|(_, ln)| t += get_power(ln));
    t
}

fn get_power(line: &str) -> u32 {
    let game_start_index = line.find(':').unwrap();
    let ln: &str = &line[game_start_index + 1..];
    let game = ln.split(';').enumerate().filter_map(|(_, sub)| {
        let round: Vec<DiceColour> = sub
            .split(',')
            .enumerate()
            .map(|(_, x)| {
                let digits: String = x.chars().filter(|c| c.is_digit(10)).collect();
                let colour: String = x.chars().filter(|c| c.is_alphabetic()).collect();
                DiceColour {
                    amount: digits.parse::<u32>().unwrap(),
                    colour,
                }
            })
            .collect::<Vec<DiceColour>>()
            .try_into()
            .unwrap();
        Some(round)
    });

    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;

    let _ = game.for_each(|r| {
        r.iter()
            .for_each(|dc: &DiceColour| match dc.colour.as_str() {
                "red" => {
                    if red_max < dc.amount {
                        red_max = dc.amount;
                    }
                }
                "green" => {
                    if green_max < dc.amount {
                        green_max = dc.amount;
                    }
                }
                "blue" => {
                    if blue_max < dc.amount {
                        blue_max = dc.amount;
                    }
                }
                _ => println!("unknow colour"),
            })
    });

    let power = red_max * green_max * blue_max;
    power
}

#[derive(Debug)]
struct DiceColour {
    amount: u32,
    colour: String,
}

#[cfg(test)]
mod tests {
    use super::get_part2_anwser;

    #[test]
    fn part2_example() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = get_part2_anwser(contents);
        assert_eq!(result, 2286);
    }
}
