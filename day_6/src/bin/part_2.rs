use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Day 6 Part 2");
    // open and parse file
    let mut file = File::open("src/input_6").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", get_part2_anwser(&contents));
}

fn get_part2_anwser(contents: &str) -> u128 {
    let td = get_time_distance(contents);
    let mut totel: u128 = 1;
    td.iter()
        .filter_map(|item| {
            let mut pos: u128 = 0;
            for i in 0..item.time {
                let check = i * (item.time - i);
                if check > item.distance {
                    pos = pos + 1;
                }
            }
            Some(pos as u128)
        })
        .for_each(|i| totel *= i);

    totel
}

fn get_time_distance(contents: &str) -> Vec<TimeDistance> {
    let first_line = contents.lines().next().unwrap_or_else(|| "");
    let last_line = contents.lines().last().unwrap_or_else(|| "");

    let time_num: u128 = first_line
        .split_whitespace()
        .filter(|c| c.chars().all(|t| t.is_digit(10)))
        .collect::<Vec<&str>>()
        .concat()
        .parse()
        .unwrap();

    let distence_num: u128 = last_line
        .split_whitespace()
        .filter(|c| c.chars().all(|t| t.is_digit(10)))
        .collect::<Vec<&str>>()
        .concat()
        .parse()
        .unwrap();

    println!("time: {:?}", time_num);
    println!("distacen: {:?}", distence_num);

    let time_distance: Vec<TimeDistance> = vec![TimeDistance {
        time: time_num,
        distance: distence_num,
    }];

    time_distance
}

struct TimeDistance {
    time: u128,
    distance: u128,
}

#[cfg(test)]
mod tests {
    use super::get_part2_anwser;

    #[test]
    fn part1_example() {
        let contents = "Time:      7  15   30
        Distance:  9  40  200";
        let result = get_part2_anwser(contents);
        assert_eq!(result, 71503);
    }
}
