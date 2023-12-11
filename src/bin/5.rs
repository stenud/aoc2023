#![allow(unused_imports)]
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/5.txt").unwrap();
    //let contents = _example();

    let seed_data: Vec<&str> = contents.split("\n\n").collect();

    let seeds: Vec<u64> = seed_data[0]
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let map_data: Vec<Vec<Vec<u64>>> = seed_data
        .iter()
        .skip(1)
        .map(|md| md
            .lines()
            .skip(1)
            .map(|l| l
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect()
            )
            .collect()
        )
        .collect();

    let closest_location_seed_a = find_closest_location_seed_a(&seeds, &map_data);
    let closest_location_seed_b = find_closest_location_seed_b(&seeds, &map_data);
    println!("a: seed {} at location {}", closest_location_seed_a.1, closest_location_seed_a.0);
    println!("b: seed {} at location {}", closest_location_seed_b.1, closest_location_seed_b.0);   
}

fn find_closest_location_seed_a(seeds: &Vec<u64>, map_data: &Vec<Vec<Vec<u64>>>) -> (u64, u64) {
let mut closest_location_seed: (u64, u64) = (0, 0);

    for seed in seeds.iter() {
        let mut next_value = seed.to_owned();

        for mapping in map_data.iter() {
            next_value = map_value(next_value, mapping);
        }

        if closest_location_seed == (0, 0) || closest_location_seed.0 > next_value {
            closest_location_seed = (next_value, seed.to_owned());
        }
    }
    closest_location_seed
}

fn find_closest_location_seed_b(seeds: &Vec<u64>, map_data: &Vec<Vec<Vec<u64>>>) -> (u64, u64) {
    let mut closest_location_seed: (u64, u64) = (0, 0);
    
        for seed_and_range in seeds.windows(2).step_by(2) {
            let seeds_in_range: Vec<u64> = (seed_and_range[0] .. seed_and_range[0] + seed_and_range[1]).collect();

            for seed in seeds_in_range {
                let mut next_value = seed;
    
                for mapping in map_data.iter() {
                    next_value = map_value(next_value, mapping);
                }
        
                if closest_location_seed == (0, 0) || closest_location_seed.0 > next_value {
                    closest_location_seed = (next_value, seed);
                }
            }
            println!("one seed pair done!");
        }
        closest_location_seed
    }

fn map_value(val: u64, map_data: &Vec<Vec<u64>>) -> u64 {
    for mapping in map_data.iter() {
        if mapping[1] <= val && val <= (mapping[1] + mapping[2]) {
            return mapping[0] + val - mapping[1];
        }
    }

    val
}

fn _example() -> String {
    "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string()
}