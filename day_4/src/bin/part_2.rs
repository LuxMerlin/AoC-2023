use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Day 4 Part 1");
    // open and parse file
    let mut file = File::open("src/input_4").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", get_part1_anwser(&contents));
}

fn get_part1_anwser(contents: &str) -> u32 {
    let games: Vec<&str> = contents
        .lines()
        .map(|line| {
            let i = line.chars().position(|c| c == ':').unwrap();
            &line[i..]
        })
        .collect();

    let mut t = 0;
    for (i, g) in games.iter().enumerate() {
        t = t + check(g, &games, i)
    }

    t
}

fn check(current_game: &str, games: &Vec<&str>, index: usize) -> u32 {
    let i = current_game.chars().position(|c| c == ':').unwrap();
    let games: Vec<Vec<&str>> = current_game[i..]
        .split('|')
        .map(|subs| subs.split(" ").collect::<Vec<&str>>())
        .collect();

    let winning_numbers = games.first().unwrap();
    let numbers = games.last().unwrap();
    let mut t = 0;
    for wn in winning_numbers {
        if numbers.contains(wn) && !wn.is_empty() {
            t += 1;
            print!("{},", wn)
        }
    }

    let dups: usize = get_score(t);
    if dups == 0 {
        return 0;
    }
    t = 0;
    let copies = &games[index..dups];
    for (i, g) in copies.iter().enumerate() {
        t = t + check(g, &games, i)
    }

    t + 1
}

fn get_score(count: u32) -> usize {
    if count == 0 {
        return 0;
    }

    1 * 2_u32.pow(count - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::get_part1_anwser;

    #[test]
    fn part1_example() {
        let contents = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = get_part1_anwser(contents);
        assert_eq!(result, 13);
    }
}
