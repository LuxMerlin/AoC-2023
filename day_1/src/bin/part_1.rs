use std::fs::File;
use std::io::prelude::*;
fn main() {
    println!("Part 1");
    let mut total: i32 = 0;
    // open and parse file
    let mut file = File::open("input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        // filter the string to only chars that are numbers
        let s: String = line.chars().filter(|c| c.is_digit(10)).collect();
        // if the string is a single char then parts the number and return
        if s.len() == 1 {
            let num_as_sting = s.to_string() + &s.to_string();
            let num = num_as_sting.parse::<i32>().unwrap();
            total += num;
        } else {
            let num_as_string = s.chars().next().expect("stirng empty").to_string()
                + &s.chars().last().expect("stirng emtpy").to_string();
            let num = num_as_string.parse::<i32>().unwrap();
            total += num;
        }
    }

    println!("Total: {}", total);
}
