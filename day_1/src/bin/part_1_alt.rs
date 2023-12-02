use std::fs::File;
use std::io::prelude::*;
fn main() {
    println!("Part 1 alt");

    // open and parse file
    let mut file = File::open("src/input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let total: u32 = contents
        .lines()
        .map(|line| {
            // filter and map the array of chars for a given line, only returning the ones that are digits
            let mut iter = line.chars().filter_map(|char| char.to_digit(10));

            // grab the first char, panic if there is nothing in the iterator
            let first = iter.next().expect("no number was found");
            // grab the last char, use the first if there is no last as a default
            let last = iter.last().unwrap_or(first);
            // turn the first into a tens, and add the units
            first * 10 + last
        })
        .sum::<u32>();

    println!("Total: {}", total);
}
