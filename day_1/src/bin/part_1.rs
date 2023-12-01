use std::fs::File;
use std::io::prelude::*;
fn main() {
    println!("Part 1");
    let mut total: i32 = 0;
    // open and parse file
    let mut file = File::open("input_1").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines(){
        let s: String = line.chars().filter(|c| c.is_digit(10)).collect();
        if s.len() == 1 {
            let num_as_sting = s.to_string() + &s.to_string();
            let num = num_as_sting.parse::<i32>().unwrap();
            total += num;
            println!("Found number: {0} : {1}", s, num);
       }else { 
           let num_as_string  = s.chars().next().expect("stirng empty").to_string() + &s.chars().last().expect("stirng emtpy").to_string();
           let num = num_as_string.parse::<i32>().unwrap();
           total += num;
           println!("Found number: {0} : {1}", s, num);
       }
    }
   

    println!("Total: {}", total);
}
