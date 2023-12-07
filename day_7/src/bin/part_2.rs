use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Day 7 Part 2");
    // open and parse file
    let mut file = File::open("src/input_7").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", get_part2_anwser(&contents));
}

fn get_part2_anwser(contents: &str) -> u32 {
    let mut hands: Vec<Hand> = contents
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(" ");
            let cards: Vec<char> = parts.next().unwrap().chars().collect::<Vec<char>>();
            Some(Hand {
                cards: cards.clone(),
                bet: parts.last().unwrap().parse::<u32>().unwrap(),
                rank: get_rank(&cards),
            })
        })
        .collect::<Vec<Hand>>();

    bubble_sort(&mut hands);

    hands
        .into_iter()
        .enumerate()
        .filter_map(|(i, h)| Some(h.bet * (i as u32 + 1)))
        .sum::<u32>()
}

fn swap(left: &Hand, right: &Hand) -> bool {
    let left_c: &Vec<char> = &left.cards;
    let right_c: &Vec<char> = &right.cards;

    if left.rank < right.rank {
        return true;
    }

    if left.rank > right.rank {
        return false;
    }

    let mut s = false;
    for (i, l) in left_c.into_iter().enumerate() {
        let l_val = conver_char_to_num(l);
        let r_val = conver_char_to_num(right_c.get(i).unwrap());

        if l_val < r_val {
            s = true;
            break;
        }

        if l_val > r_val {
            s = false;
            break;
        }

        // if eq check next num
    }

    s
}

fn conver_char_to_num(c: &char) -> u32 {
    if let Some(number) = c.to_digit(10) {
        number
    } else {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => 0,
        }
    }
}

fn bubble_sort(arr: &mut Vec<Hand>) {
    let mut new_len: usize;
    let mut len = arr.len();
    // Outer loop
    loop {
        new_len = 0;
        // Inner loop
        for i in 1..len {
            if swap(&arr[i], &arr[i - 1]) {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == 0 {
            break;
        }
        len = new_len;
    }
}

fn get_rank(cards: &Vec<char>) -> u8 {
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for &ch in cards {
        let count = char_count.entry(ch).or_insert(0);
        *count += 1;
    }
    //five of a kind
    if char_count.values().any(|c| c.eq(&5)) {
        return 7;
    }

    //four of a kind
    if char_count.values().any(|c| c.eq(&4)) {
        return 6;
    }

    //full house
    if char_count.values().any(|c| c.eq(&3)) && char_count.values().any(|c| c.eq(&2)) {
        return 5;
    }

    //three of a kind
    if char_count.values().any(|c| c.eq(&3)) {
        return 4;
    }
    //two of a kind
    if char_count.values().filter(|c| c.eq(&&2)).count() == 2 {
        return 3;
    }

    //two of a kind
    if char_count.values().filter(|c| c.eq(&&2)).count() == 1 {
        return 2;
    }

    return 1;
}

struct Hand {
    cards: Vec<char>,
    bet: u32,
    rank: u8,
}

#[cfg(test)]
mod tests {
    use super::get_part2_anwser;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn part1_example_reddit() {
        let mut file = File::open("src/input_7_test_one").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let result = get_part2_anwser(&contents);
        assert_eq!(result, 6592);
    }

    #[test]
    fn part1_example_aoc() {
        let mut file = File::open("src/input_7_test_two").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let result = get_part2_anwser(&contents);
        assert_eq!(result, 6440);
    }
}
