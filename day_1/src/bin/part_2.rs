use std::fs::File;
use std::io::prelude::*;

const NUMS_AS_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!("Part 1 alt");

    // open and parse file
    let mut file = File::open("src/input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Total: {}", get_total(&contents));
}

fn get_total(content: &str) -> u32 {
    let total: u32 = content
        .lines()
        .map(|line| {
            let mut iter = line.chars().enumerate().filter_map(|(index, char)| {
                // if the char is a number just return
                if let Some(num) = char.to_digit(10) {
                    Some(num)
                } else {
                    let num_string = &line[index..];
                    // using the array if numbers as strings, iter over
                    // and filter till there is a match
                    // use the index to match the number and add 1
                    NUMS_AS_STRINGS
                        .iter()
                        .enumerate()
                        .filter_map(|(index, num_str)| {
                            num_string.starts_with(num_str).then_some(index as u32 + 1)
                        })
                        .next()
                }
            });
            // below is the same as the part_1_alt
            let first = iter.next().expect("no number was found");
            let last = iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>();

    total
}
