use std::fs::File;
use std::io::prelude::*;

const COLOUR_TOTALS: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn main() {
    println!("Day 2 Part 1");
    // open and parse file
    let mut file = File::open("src/input_2").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", get_part1_anwser(&contents));
}

fn get_part1_anwser(contents: &str) -> u32 {
    let mut t = 0;
    let _ = contents.lines().enumerate().for_each(|(index, ln)| {
        let check = is_possible(&ln);
        if check {
            println!("passed {}", index as u32 + 1);
            t += index as u32 + 1;
        }
    });
    t
}

fn is_possible(line: &str) -> bool {
    let game_start_index = line.find(':').unwrap();
    let ln = &line[game_start_index + 1..];
    let faile_checks = ln.split(';').enumerate().filter(|(_, sub)| {
        let dice = sub
            .split(',')
            .enumerate()
            .map(|(_, x)| {
                let digits: String = x.chars().filter(|c| c.is_digit(10)).collect();
                let colour: String = x.chars().filter(|c| c.is_alphabetic()).collect();
                (digits.parse::<i32>().unwrap(), colour)
            })
            .enumerate()
            .filter(|(_, (n, c))| is_dice_count_invalid(n, c));

        dice.count() > 0
    });
    let count = faile_checks.count();
    return count == 0;
}

fn is_dice_count_invalid(n: &i32, c: &str) -> bool {
    match c {
        "red" => n > &12,
        "green" => n > &13,
        "blue" => n > &14,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::get_part1_anwser;

    #[test]
    fn part1_example() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = get_part1_anwser(contents);
        assert_eq!(result, 8);
    }
}
