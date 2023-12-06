#![allow(unused_imports)]
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct NumberAndPos {
    number: u32,
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct GearSymbols {
    line: usize,
    pos: u32,
}

fn main() {
    let contents = fs::read_to_string("input/3.txt").unwrap();
    //let contents = _example();
    
    let mut valid_numbers: Vec<u32> = Vec::new();
    let mut valid_numbers_lines: HashMap<usize, Vec<NumberAndPos>> = HashMap::new();
    let mut gear_symbols: Vec<GearSymbols> = Vec::new();

    for (i, _line) in contents.lines().enumerate() {
        let three_lines: Vec<&str> = contents.lines().skip(i).take(3).collect();
        if i == 0 { 
            //check first line...
            let (number_arr, gear_symbols_pos) = check_for_numbers(three_lines[0]);
            let (valid_numbers_in_line, valid_numbers_pos) = 
                check_for_symbol(number_arr, &three_lines[..2]);
            valid_numbers_lines.insert(i+1, valid_numbers_pos);
            add_to_valid_numbers(&mut valid_numbers, &valid_numbers_in_line);
            add_to_gear_symbols(&mut gear_symbols, &gear_symbols_pos, i+1);
            //...and middle line
        };
        if three_lines.len() >= 2 {
            //check middle line (and last line of 2 for last line)
            let (number_arr, gear_symbols_pos) = check_for_numbers(three_lines[1]);
            let (valid_numbers_in_line, valid_numbers_pos) = 
                check_for_symbol(number_arr, &three_lines);
            valid_numbers_lines.insert(i+2, valid_numbers_pos);
            add_to_valid_numbers(&mut valid_numbers, &valid_numbers_in_line);
            add_to_gear_symbols(&mut gear_symbols, &gear_symbols_pos, i+2);
        };
    }
    let valid_gears = check_valid_gears(valid_numbers_lines, gear_symbols);
    let valid_gears: u32 = valid_gears.iter().sum();
    let valid_numbers: u32 = valid_numbers.iter().sum();
    println!("a: {}", valid_numbers);
    println!("b: {}", valid_gears);
}

fn check_for_numbers(line: &str) -> (Vec<NumberAndPos>, Vec<u32>) {
    let mut number = String::new();
    let mut found_number = false;
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut number_arr: Vec<NumberAndPos> = Vec::new();
    let mut gear_symbols_pos: Vec<u32> = Vec::new();

    for (i, char) in line.chars().enumerate() {
        if char == '*' {
            gear_symbols_pos.push(i as u32);
        }

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
    (number_arr, gear_symbols_pos)
}

fn check_for_symbol(number_arr: Vec<NumberAndPos>, lines: &[&str]) -> (Vec<u32>, Vec<NumberAndPos>) {

    let mut valid_numbers: Vec<u32> = Vec::new();
    let mut valid_numbers_with_pos: Vec<NumberAndPos> = Vec::new();

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
                    valid_numbers_with_pos.push(number);
                    continue 'number;
                }
            };
        };

    };
    (valid_numbers, valid_numbers_with_pos)
}

fn add_to_valid_numbers(valid_numbers: &mut Vec<u32>, valid_numbers_in_line: &Vec<u32>) {
    for number in valid_numbers_in_line.iter() {
        valid_numbers.push(number.clone());
    }
}

fn add_to_gear_symbols(gear_symbols: &mut Vec<GearSymbols>, gear_symbols_pos: &Vec<u32>, line: usize) {
    for pos in gear_symbols_pos.iter() {
        gear_symbols.push(
            GearSymbols {
                line,
                pos: pos.clone(), 
            }
        );
    }
}

fn check_valid_gears(valid_numbers_lines: HashMap<usize, Vec<NumberAndPos>>, gear_symbols: Vec<GearSymbols>)
    -> Vec<u32> {
        let mut valid_gears: Vec<u32> = Vec::new();

        for gear in gear_symbols.iter() {
            let line = gear.line;
            let pos = gear.pos;
            let mut count_numbers: Vec<u32> = Vec::new();

            for i in 0..3 {
                match valid_numbers_lines.get(&(line + i - 1)) {
                    Some(numbers) => {
                        for number in numbers {
                            let start_number = number.start as i32;
                            if (start_number - 1) <= pos as i32 && number.end >= pos {
                                count_numbers.push(number.number);
                            }
                        }
                    },
                    None => (),
                };
            };
            if count_numbers.len() == 2 {
                valid_gears.push( count_numbers[0] * count_numbers[1] );
            }
        };
        valid_gears
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