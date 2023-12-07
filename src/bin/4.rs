#![allow(unused_imports)]
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/4.txt").unwrap();
    //let contents = _example();

    let mut points = 0;
    let num_lines = contents.lines().count();
    let mut scratchcard_count: Vec<u32> = vec![1;num_lines];

    for (card_nr, line) in contents.lines().enumerate() {
        let card_nr = card_nr;
        let mut card_points: u32 = 0;

        if let Some((winning_numbers_str, my_numbers_str)) = line
            .split(": ")
            .collect::<Vec<_>>()
            .get(1)
            .map(|s| {
                let numbers: Vec<&str> = s.split(" | ").collect();
                (numbers[0], numbers[1])
            }) {
                let winning_numbers: Vec<u32> = winning_numbers_str
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();
                let my_numbers: Vec<u32> = my_numbers_str
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();

                for number in winning_numbers.iter() {
                    if my_numbers.contains(number) {
                        card_points += 1;
                    }
                }
                points += match card_points{
                    0 => 0,
                    1 => 1,
                    x => 1<<(x-1), //double value
                };

                for point in 0..card_points {
                    scratchcard_count[card_nr + point as usize + 1] += scratchcard_count[card_nr];
                }
        }
    }
    println!("a: {}", points);
    println!("b: {}", scratchcard_count.iter().sum::<u32>());
}

fn _example() -> String {
    "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string()
}