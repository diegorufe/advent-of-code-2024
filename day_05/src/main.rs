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

fn calculate_valid_center_values(rules: &HashMap<i32, HashSet<i32>>, data: &Vec<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;

    for values in data {
        total = total + find_center_value(rules, values);
    }

    return total;
}

fn is_valid(rules: &HashMap<i32, HashSet<i32>>, values: &Vec<i32>) -> bool {
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

    return valid;
}

fn find_center_value(rules: &HashMap<i32, HashSet<i32>>, values: &Vec<i32>) -> i32 {
    let mut center_value = 0;

    if is_valid(rules, values) {
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

fn calculate_invalid_center_values(
    rules: &HashMap<i32, HashSet<i32>>,
    data: &Vec<Vec<i32>>,
) -> i32 {
    let mut total: i32 = 0;

    for values in data {
        if !is_valid(rules, values) {
            total = total + calculate_invalid_value(rules, values);
        }
    }

    return total;
}

fn calculate_invalid_value(rules: &HashMap<i32, HashSet<i32>>, values: &Vec<i32>) -> i32 {
    let mut valid_values: Vec<i32> = Vec::new();
    let mut invalid_values = values.clone();

    while valid_values.len() != values.len() {
        let mut remove_index: isize = -1;

        for (index, value) in invalid_values.iter().enumerate() {
            if invalid_values.len() == 1 {
                valid_values.push(*value);
                break;
            }

            let mut invalid_values_check = invalid_values.clone();
            invalid_values_check.remove(index);

            if contains_values(*value, rules, &invalid_values_check) {
                remove_index = index as isize;
                valid_values.push(*value);
                break;
            }
        }

        if remove_index >= 0 {
            invalid_values.remove(remove_index as usize);
        }
    }

    return find_center_value(rules, &valid_values);
}

fn contains_values(value: i32, rules: &HashMap<i32, HashSet<i32>>, values: &Vec<i32>) -> bool {
    if !rules.contains_key(&value) {
        return false;
    }

    let sequences = rules.get(&value).expect("unable to find rules");

    return validate_sequence(sequences, values.to_vec());
}

fn history_example_part_one() {
    let start = Instant::now();
    let (rules, data): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) =
        read_data("./src/history_example_part_one_data.txt");
    let total = calculate_valid_center_values(&rules, &data);
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
    let total = calculate_valid_center_values(&rules, &data);
    let duration = start.elapsed();
    print!("History part one. Total {}. Time: {:?} \n", total, duration)
}

fn history_example_part_two() {
    let start = Instant::now();
    let (rules, data): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) =
        read_data("./src/history_example_part_two_data.txt");
    let total = calculate_invalid_center_values(&rules, &data);
    let duration = start.elapsed();
    print!(
        "History example part two. Total {}. Time: {:?} \n",
        total, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let (rules, data): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) =
        read_data("./src/history_part_two_data.txt");
    let total = calculate_invalid_center_values(&rules, &data);
    let duration = start.elapsed();
    print!("History part two. Total {}. Time: {:?} \n", total, duration)
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
