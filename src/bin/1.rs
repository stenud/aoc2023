#[allow(unused_imports)]
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/1.txt").unwrap();
    //let contents = _example_a();
    //let contents = _example_b();

    let contents_b = replace_written_numbers(&contents);

    println!("1a: {}", find_sum(&contents));
    println!("1b: {}", find_sum(&contents_b))
}

fn find_sum(contents: &str) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let mut number_str: String = String::new();

        for c in line.chars() {
            if c.is_ascii_digit() {
                number_str.push(c);
                break;
            }
        }
        
        //reverse
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                number_str.push(c);
                break;
            }
        }
        
        numbers.push(number_str.parse().unwrap());
    }

    numbers.iter().sum()
}

fn replace_written_numbers(contents: &str) -> String {
    let contents = contents.replace("one", "o1e");
    let contents = contents.replace("two", "t2");
    let contents = contents.replace("three", "t3e");
    let contents = contents.replace("four", "4");
    let contents = contents.replace("five", "5e");
    let contents = contents.replace("six", "6");
    let contents = contents.replace("seven", "7n");
    let contents = contents.replace("eight", "e8");
    let contents = contents.replace("nine", "9");

    println!("{contents}");
    contents
}

fn _example_a() -> String {
    "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
".to_string()
}

fn _example_b() -> String {
    "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
".to_string()
}

