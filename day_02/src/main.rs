use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn convert_to_vector(input_file_name: &str) -> Vec<Vec<i32>> {
    let mut vect_input: Vec<Vec<i32>> = Vec::new();

    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let values: Vec<i32> = line
            .expect("Error read line")
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vect_input.push(values);
    }

    return vect_input;
}

fn calculate_secure_codes(input_values: Vec<Vec<i32>>) -> i32 {
    let mut secure_coodes: i32 = 0;

    for values in input_values.iter() {
        let valid = validate_secure_codes(values);

        if valid {
            secure_coodes = secure_coodes + 1;
        }
    }

    return secure_coodes;
}

fn validate_secure_codes(input_values: &Vec<i32>) -> bool {
    let mut valid: bool = true;
    let mut increment: bool = false;
    let mut old_value: i32 = 0;

    for (index, value) in input_values.iter().enumerate() {
        let result = *value - old_value;
        let result_abs = result.abs();
        let mut increment_value: bool = result > 0;

        match index {
            0 => {
                old_value = *value;
                continue;
            }
            1 => {
                increment = result > 0;
                increment_value = increment;
            }
            _ => {}
        }

        if result_abs <= 0 || result_abs > 3 {
            valid = false;
        } else if increment != increment_value {
            valid = false;
        }

        if !valid {
            break;
        }

        old_value = *value;
    }

    return valid;
}

fn history_example_part_one() {
    let start = Instant::now();
    let input_values = convert_to_vector("./src/history_example_part_one_data.txt");
    let secure_codes = calculate_secure_codes(input_values);
    let duration = start.elapsed();
    print!(
        "History example part one. Secure codes {}. Time: {:?}  \n",
        secure_codes, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let input_values = convert_to_vector("./src/history_part_one_data.txt");
    let secure_codes = calculate_secure_codes(input_values);
    let duration = start.elapsed();
    print!(
        "History part one. Secure codes {}. Time: {:?}  \n",
        secure_codes, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
}
