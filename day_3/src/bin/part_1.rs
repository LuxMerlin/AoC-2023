use std::fs::File;
use std::io::prelude::*;
fn main() {
    println!("Day 3 Part 1");
    // open and parse file
    let mut file = File::open("src/input_3").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("answer = {}", get_part1_anwser(&contents));
}

fn get_part1_anwser(contents: &str) -> u32 {
    let mut working_total: u32 = 0;

    let flat = contents.chars().collect::<Vec<char>>();
    println!("{:?}", flat);
    let width: usize = contents.lines().next().unwrap().len() + 1;
    let height: usize = contents.lines().count();
    println!("h{0}, w{1}", height, width);

    let mut current_num: String = "".into();
    let mut curret_has_symbol = false;
    for (i, c) in flat.iter().enumerate() {
        if !c.is_digit(10) {
            if curret_has_symbol {
                println!("adding {}", current_num);

                working_total = working_total + current_num.parse::<u32>().unwrap();
                curret_has_symbol = false;
            }

            current_num = "".into();
        } else {
            current_num = current_num + &c.to_string();
            //  println!("{}", c);
            //println!("{}", current_num);
            let neighbors = get_neighbors(i, width, height);
            let has_symbol = check_neighbors_for_symbols(&neighbors, &flat);
            if has_symbol {
                curret_has_symbol = has_symbol;
            }
            // println!("neighbors {:?}", &neighbors);
            //println!("has_symbol {:?}", has_symbol);
        }
    }
    working_total
}

fn check_neighbors_for_symbols(neighbors: &Vec<usize>, full: &Vec<char>) -> bool {
    let mut has_symbol = false;
    for i in neighbors {
        let char = full[*i];
        if char.is_digit(10) || char.is_alphabetic() || char == '.' || char.is_control() {
            has_symbol = false;
        } else {
            has_symbol = true;
            break;
        }
    }

    has_symbol
}

fn get_neighbors(index: usize, grid_width: usize, grid_height: usize) -> Vec<usize> {
    let row = index / grid_width;
    let col = index % grid_width;

    let mut neighbors = Vec::new();

    // Iterate over neighboring rows
    for i in (row.saturating_sub(1))..=(row + 1).min(grid_height - 1) {
        // Iterate over neighboring columns
        for j in (col.saturating_sub(1))..=(col + 1).min(grid_width - 1) {
            // Convert 2D indices back to 1D index
            let neighbor_index = i * grid_width + j;
            if neighbor_index != index {
                neighbors.push(neighbor_index);
            }
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::get_part1_anwser;

    #[test]
    fn part1_example() {
        let contents = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let result = get_part1_anwser(contents);
        assert_eq!(result, 4361);
    }
}
