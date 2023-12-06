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

fn get_part2_anwser(contents: &str) -> u32 {
    let mut t = 0;

    t
}

#[cfg(test)]
mod tests {
    use super::get_part2_anwser;

    #[test]
    fn part1_example() {
        let contents = "Time:      7  15   30
        Distance:  9  40  200";
        let result = get_part2_anwser(contents);
        assert_eq!(result, 288);
    }
}
