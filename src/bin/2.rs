#![allow(unused_imports)]
use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input/2.txt").unwrap();
    //let contents = _example();

    let (games_passed, min_colors) = analyze_games(&contents);
    println!("a: {}", games_passed);
    println!("b: {}", min_colors);
}

fn analyze_games(contents: &str) -> (u32, u32) {
    let mut games_passed: Vec<u32> = Vec::new();
    let mut power_cubes: Vec<u32> = Vec::new();

    for (game_nr, line) in contents.lines().enumerate() {
        let game_nr = game_nr + 1;

        let split = if game_nr < 10 { 
            8
        } else if game_nr < 100 { 
            9
        } else { 
            10
        };

        let (_game_and_number, line) = line.split_at(split);

        let mut min_colors: HashMap<&str, u32> = HashMap::new();
        min_colors.insert(&"red", 0);
        min_colors.insert(&"green", 0);
        min_colors.insert(&"blue", 0);
        let mut passed = true;

        for set in line.split("; ") {
            let mut set_arr: Vec<(u32, &str)> = Vec::new();
            for color in set.split(", ") {
                let color: Vec<&str> = color.split(' ').collect();
                set_arr.push((color[0].parse().unwrap(), color[1]));
            }

            for (number, color) in set_arr.iter() {
                match color {
                    &"red" => if number.to_owned() > 12 { passed = false },
                    &"green" => if number.to_owned() > 13 { passed = false },
                    &"blue" => if number.to_owned() > 14 { passed = false },
                    _ => (),
                }

                //if min_colors.get(color).unwrap() < number { min_colors.insert("red", 0) };
                if min_colors.get(color).unwrap().to_owned() < number.to_owned() { min_colors.insert(color, number.to_owned()); };
            }
        }
        if passed { games_passed.push(game_nr as u32) };
        power_cubes.push(
            min_colors.get(&"red").unwrap().to_owned() *
            min_colors.get(&"green").unwrap().to_owned() *
            min_colors.get(&"blue").unwrap().to_owned()
        );
    }
    (games_passed.iter().sum(), power_cubes.iter().sum())
}

fn _example() -> String {
    "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
}