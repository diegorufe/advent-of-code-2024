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
        if *value == 0 {
            stones.push(1);
            continue;
        }

        let len = (*value as f64).log(10.0).floor() as i64 + 1;

        if len % 2 == 0 {
            let divisor = 10_i64.pow((len / 2 as i64).try_into().unwrap());
            let first = value / divisor;
            let second = value % divisor;

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
