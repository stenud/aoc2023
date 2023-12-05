#![allow(unused_imports)]
use std::fs;

#[derive(Debug)]
struct NumberAndPos {
    number: u32,
    start: u32,
    end: u32,
}

fn main() {
    let contents = fs::read_to_string("input/3.txt").unwrap();
    //let contents = _example();
    
    let mut valid_numbers: Vec<u32> = Vec::new();

    for (i, _line) in contents.lines().enumerate() {
        let three_lines: Vec<&str> = contents.lines().skip(i).take(3).collect();
        if i == 0 { 
            //check first line...
            let number_arr = check_for_numbers(three_lines[0]);
            let valid_numbers_in_line = check_for_symbol(&number_arr, &three_lines[..2]);
            add_to_valid_numbers(&mut valid_numbers, &valid_numbers_in_line);
            //...and middle line
            let number_arr = check_for_numbers(three_lines[1]);
            let valid_numbers_in_line = check_for_symbol(&number_arr, &three_lines);
            add_to_valid_numbers(&mut valid_numbers, &valid_numbers_in_line);
        } else if three_lines.len() == 3 {
            //check middle line
            let number_arr = check_for_numbers(three_lines[1]);
            let valid_numbers_in_line = check_for_symbol(&number_arr, &three_lines);
            add_to_valid_numbers(&mut valid_numbers, &valid_numbers_in_line);
        } else if three_lines.len() == 2 {
            //check last line of 2
            let number_arr = check_for_numbers(three_lines[1]);
            let valid_numbers_in_line = check_for_symbol(&number_arr, &three_lines);
            add_to_valid_numbers(&mut valid_numbers, &valid_numbers_in_line);
        } else {
            ();
        }
    }
    let valid_numbers: u32 = valid_numbers.iter().sum();
    println!("a: {}", valid_numbers);
}

fn check_for_numbers(line: &str) -> Vec<NumberAndPos> {
    let mut number = String::new();
    let mut found_number = false;
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut number_arr: Vec<NumberAndPos> = Vec::new();

    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            number.push(char);
            if !found_number {
                start = i as u32;
                found_number = true;
            }
        } else {
            if found_number {
                end = i as u32;
                found_number = false;
                let number_u32: u32 = number.parse().unwrap();
                number_arr.push(
                    NumberAndPos { 
                        number: number_u32,
                        start,
                        end,
                    }
                );
                number = "".to_string();
            }
        }
    }
    if !number.is_empty() {
        end = start + number.len() as u32;
        let number_u32: u32 = number.parse().unwrap();
        number_arr.push(
            NumberAndPos { 
                number: number_u32,
                start,
                end,
            }
        );
    }
    number_arr
}

fn check_for_symbol(number_arr: &Vec<NumberAndPos>, lines: &[&str]) -> Vec<u32> {

    let mut valid_numbers: Vec<u32> = Vec::new();

    'number: for number in number_arr {
        let start = match number.start {
            0 => { 0 },
            _ => { number.start - 1 },
        };
        let len = number.end + 1 - start;

        for line in lines.iter() {
            let chars_to_check: String = line
                .chars()
                .skip(start as usize)
                .take(len as usize)
                .collect();

            for char in chars_to_check.chars() {
                if !(char.is_ascii_digit() || char == '.') {
                    valid_numbers.push(number.number);
                    continue 'number;
                }
            };
        };
    };
    valid_numbers
}

fn add_to_valid_numbers(valid_numbers: &mut Vec<u32>, valid_numbers_in_line: &Vec<u32>) {
    for number in valid_numbers_in_line.iter() {
        valid_numbers.push(number.clone());
    }
}

fn _example() -> String {
    "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string()
}