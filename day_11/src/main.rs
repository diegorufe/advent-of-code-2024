use std::{fs, time::Instant};

fn read_data(input_file_name: &str) -> Vec<i64> {
    let content = fs::read_to_string(input_file_name).unwrap();
    let values: Vec<i64> = content
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    return values;
}

fn calculate(data: Vec<i64>) -> Vec<i64> {
    let mut stones: Vec<i64> = Vec::new();

    for value in data.iter() {
        let value_str = value.to_string();
        if *value == 0 {
            stones.push(1);
        } else if value_str.len() % 2 == 0 {
            let middle = value_str.len() / 2;
            let first: i64 = value_str[0..middle].to_string().parse().unwrap();
            let second: i64 = value_str[middle..].to_string().parse().unwrap();

            stones.push(first);
            stones.push(second);
        } else {
            stones.push(*value * 2024);
        }
    }

    return stones;
}

fn calculate_stones(times: i64, data: Vec<i64>) -> Vec<i64> {
    let mut stones: Vec<i64> = data;

    for _ in 0..times {
        stones = calculate(stones);
    }

    return stones;
}

fn history_example_part_one() {
    let start = Instant::now();
    let data: Vec<i64> = read_data("./src/history_example_part_one_data.txt");
    let stones: Vec<i64> = calculate_stones(6, data);
    let duration = start.elapsed();
    print!(
        "History example part one. Total {}. Time: {:?}  \n",
        stones.len(),
        duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let data: Vec<i64> = read_data("./src/history_part_one_data.txt");
    let stones: Vec<i64> = calculate_stones(25, data);
    let duration = start.elapsed();
    print!(
        "History part one. Total {}. Time: {:?}  \n",
        stones.len(),
        duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one()
}
