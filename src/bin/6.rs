fn main() {
    let race_data_a = _input_a(); let race_data_b = _input_b();
    //let race_data_a = _example_a(); let race_data_b = _example_b();

    let multiplied_records_a = calculate(&race_data_a);
    let multiplied_records_b = calculate(&race_data_b);

    println!("a: {}", multiplied_records_a);
    println!("b: {}", multiplied_records_b);
}

fn calculate(race_data: &Vec<(u64, u64)>) -> u64 {
    let mut multiplied_records: u64 = 1;

    for (time, distance) in race_data.iter() {
        let mut num_ways_to_win = 0;

        for t in 1..*time {
            if t * (time - t) > *distance { 
                num_ways_to_win += 1 
            };
        }

        multiplied_records *= num_ways_to_win;
    }
    multiplied_records
}

fn _example_a() -> Vec<(u64, u64)> {
    let race_data: Vec<(u64, u64)> = vec![(7, 9), (15, 40), (30, 200)];
    race_data
}

fn _example_b() -> Vec<(u64, u64)> {
    let race_data: Vec<(u64, u64)> = vec![(71530, 940200)];
    race_data
}

fn _input_a() -> Vec<(u64, u64)> {
    let race_data: Vec<(u64, u64)> = vec![(48, 255), (87, 1288), (69, 1117), (81, 1623)];
    race_data
}

fn _input_b() -> Vec<(u64, u64)> {
    let race_data: Vec<(u64, u64)> = vec![(48876981, 255128811171623)];
    race_data
}