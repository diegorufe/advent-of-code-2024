use std::{collections::HashMap, fs, time::Instant};

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

fn calculate_stone(stone: i64, cache: &mut HashMap<i64, Vec<i64>>) -> Vec<i64> {
    if let Some(result) = cache.get(&stone) {
        return result.clone();
    }

    let mut result: Vec<i64> = Vec::new();

    if stone == 0 {
        result.push(1);
    } else {
        let len = (stone as f64).log(10.0).floor() as i64 + 1;

        if len % 2 == 0 {
            let divisor = 10_i64.pow((len / 2 as i64).try_into().unwrap());
            let first = stone / divisor;
            let second = stone % divisor;

            result.push(first);
            result.push(second);
        } else {
            result.push(stone * 2024);
        }
    }

    cache.insert(stone, result.clone());

    return result;
}

fn calculate_stones_cache(levels: usize, data: Vec<i64>) -> HashMap<i64, i64> {
    let mut stone_count: HashMap<i64, i64> = HashMap::new();

    for v in data {
        stone_count.insert(v, 1);
    }

    let mut cache = HashMap::new();

    for _ in 0..levels {
        let mut new_stone_count = HashMap::new();

        for (&stone, &count) in stone_count.iter() {
            let transformed = calculate_stone(stone, &mut cache);

            for t in transformed {
                *new_stone_count.entry(t).or_insert(0) += count;
            }
        }

        stone_count = new_stone_count;
    }

    return stone_count;
}

fn history_example_part_two() {
    let start = Instant::now();
    let data: Vec<i64> = read_data("./src/history_example_part_two_data.txt");
    let stone_count: HashMap<i64, i64> = calculate_stones_cache(6, data);
    let mut stones: i64 = 0;

    for (_, &count) in stone_count.iter() {
        stones += count;
    }

    let duration = start.elapsed();
    print!(
        "History example part two. Total {}. Time: {:?}  \n",
        stones, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let data: Vec<i64> = read_data("./src/history_part_two_data.txt");
    let stone_count: HashMap<i64, i64> = calculate_stones_cache(75, data);
    let mut stones: i64 = 0;

    for (_, &count) in stone_count.iter() {
        stones += count;
    }

    let duration = start.elapsed();
    print!(
        "History part two. Total {}. Time: {:?}  \n",
        stones, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
