use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_data(input_file_name: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut data: Vec<Vec<i32>> = Vec::new();

    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);
    let mut is_rule: bool = true;

    for line in reader.lines() {
        let data_line = line.expect("Unable to read line");

        if data_line.trim().is_empty() {
            is_rule = false;
            continue;
        }

        if is_rule {
            let values: Vec<i32> = data_line.split("|").map(|s| s.parse().unwrap()).collect();

            rules
                .entry(values[0])
                .or_insert(HashSet::new())
                .insert(values[1]);
        } else {
            data.push(data_line.split(",").map(|s| s.parse().unwrap()).collect());
        }
    }

    return (rules, data);
}

fn calculate_valid_center_values(rules: &HashMap<i32, HashSet<i32>>, data: Vec<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    for values in data {
        total = total + find_center_value(rules, values);
    }

    return total;
}

fn find_center_value(rules: &HashMap<i32, HashSet<i32>>, values: Vec<i32>) -> i32 {
    let mut valid: bool = true;

    for (index, value) in values.iter().enumerate() {
        if index == (values.len() - 1) {
            continue;
        }

        if !rules.contains_key(value) {
            valid = false;
            break;
        }

        valid = validate_sequence(
            rules.get(value).expect("Rules not found"),
            values[index + 1..].to_vec(),
        );

        if !valid {
            break;
        }
    }

    let mut center_value = 0;

    if valid {
        let center_index = values.len() / 2;
        center_value = values[center_index];
    }

    return center_value;
}

fn validate_sequence(sequences: &HashSet<i32>, values: Vec<i32>) -> bool {
    let mut valid: bool = true;

    for value in values {
        valid = sequences.contains(&value);

        if !valid {
            break;
        }
    }

    return valid;
}

fn history_example_part_one() {
    let start = Instant::now();
    let (rules, data): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) =
        read_data("./src/history_example_part_one_data.txt");
    let total = calculate_valid_center_values(&rules, data);
    let duration = start.elapsed();
    print!(
        "History example part one. Total {}. Time: {:?} \n",
        total, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let (rules, data): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) =
        read_data("./src/history_part_one_data.txt");
    let total = calculate_valid_center_values(&rules, data);
    let duration = start.elapsed();
    print!("History part one. Total {}. Time: {:?} \n", total, duration)
}

fn main() {
    history_example_part_one();
    history_part_one();
}
