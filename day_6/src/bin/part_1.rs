use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Day 6 Part 1");
    // open and parse file
    let mut file = File::open("src/input_6").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", get_part1_anwser(&contents));
}

fn get_part1_anwser(contents: &str) -> u32 {
    let td = get_time_distance(contents);
    let mut totel: u32 = 1;
    td.iter()
        .filter_map(|item| {
            let mut pos = 0;
            for i in 0..item.time {
                let check = i * (item.time - i);
                if check > item.distance {
                    pos = pos + 1;
                }
            }
            Some(pos as u32)
        })
        .for_each(|i| totel *= i);

    totel
}

fn get_time_distance(contents: &str) -> Vec<TimeDistance> {
    let first_line = contents.lines().next().unwrap_or_else(|| "");
    let last_line = contents.lines().last().unwrap_or_else(|| "");

    let time_num: Vec<u32> = first_line
        .split_whitespace()
        .filter(|c| c.chars().all(|t| t.is_digit(10)))
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<u32>().unwrap())
        .collect();

    let distence_num: Vec<u32> = last_line
        .split_whitespace()
        .filter(|c| c.chars().all(|t| t.is_digit(10)))
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", time_num);
    println!("{:?}", distence_num);

    let time_distance: Vec<TimeDistance> = time_num
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            Some(TimeDistance {
                time: *t,
                distance: distence_num[i],
            })
        })
        .collect();

    time_distance
}

struct TimeDistance {
    time: u32,
    distance: u32,
}

#[cfg(test)]
mod tests {
    use super::get_part1_anwser;

    #[test]
    fn part1_example() {
        let contents = "Time:      7  15   30
        Distance:  9  40  200";
        let result = get_part1_anwser(contents);
        assert_eq!(result, 288);
    }
}
