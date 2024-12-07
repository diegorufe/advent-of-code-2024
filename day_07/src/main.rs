use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

struct Calibration {
    result: i64,
    operands: Vec<i64>,
}

impl Calibration {
    fn new(result: i64, operands: Vec<i64>) -> Calibration {
        return Calibration { result, operands };
    }

    fn validate_operations(
        &self,
        index: usize,
        accumulate: i64,
        check_all_operations: bool,
    ) -> bool {
        let mut accumulate_operations_sum: i64 = accumulate;
        let mut accumulate_operations_mult: i64 = accumulate;
        let mut accumulate_operations_comb: i64 = accumulate;
        let valid_sum;
        let valid_mult;
        let mut valid_comb = false;

        if index == 0 {
            accumulate_operations_sum = self.operands[index];
            accumulate_operations_mult = self.operands[index];
            accumulate_operations_comb = self.operands[index];

            valid_sum = self.validate_operations(
                index + 1,
                accumulate_operations_sum,
                check_all_operations,
            );
            valid_mult = self.validate_operations(
                index + 1,
                accumulate_operations_mult,
                check_all_operations,
            );

            if check_all_operations {
                valid_comb = self.validate_operations(
                    index + 1,
                    accumulate_operations_comb,
                    check_all_operations,
                );
            }
        } else if index < self.operands.len() {
            accumulate_operations_sum += self.operands[index];
            accumulate_operations_mult *= self.operands[index];

            valid_sum = self.validate_operations(
                index + 1,
                accumulate_operations_sum,
                check_all_operations,
            );
            valid_mult = self.validate_operations(
                index + 1,
                accumulate_operations_mult,
                check_all_operations,
            );

            if check_all_operations {
                let operand_value_str = self.operands[index].to_string();
                let operand_comb_value_str =
                    accumulate_operations_comb.to_string() + "" + &operand_value_str;
                let operand_comb_value: i64 = operand_comb_value_str.parse().unwrap();
                accumulate_operations_comb = operand_comb_value;
                valid_comb = self.validate_operations(
                    index + 1,
                    accumulate_operations_comb,
                    check_all_operations,
                );
            }
        } else {
            valid_sum = self.result.eq(&accumulate_operations_sum);
            valid_mult = self.result.eq(&accumulate_operations_mult);
            valid_comb = check_all_operations && self.result.eq(&accumulate_operations_comb);
        }

        return valid_sum || valid_mult || (check_all_operations && valid_comb);
    }

    fn validate(&self, check_all_operations: bool) -> bool {
        return self.validate_operations(0, 0, check_all_operations);
    }
}

fn read_data(input_file_name: &str) -> Vec<Calibration> {
    let mut calibrations: Vec<Calibration> = Vec::new();
    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_value = line.expect("Unable to read line");
        let split_values: Vec<&str> = line_value.split(":").collect();
        let result: i64 = split_values[0].parse::<i64>().unwrap();
        let operands: Vec<i64> = split_values[1]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let calibration = Calibration::new(result, operands);
        calibrations.push(calibration);
    }

    return calibrations;
}

fn sum_result_valid_calibrations(
    calibrations: Vec<Calibration>,
    check_all_operations: bool,
) -> i64 {
    let mut sum_valid_calibrations: i64 = 0;

    for calibration in calibrations {
        if calibration.validate(check_all_operations) {
            sum_valid_calibrations += calibration.result;
        }
    }

    return sum_valid_calibrations;
}

fn history_example_part_one() {
    let start = Instant::now();
    let calibrations = read_data("./src/history_example_part_one_data.txt");
    let sum_valid_calibrations = sum_result_valid_calibrations(calibrations, false);
    let duration = start.elapsed();
    print!(
        "History example part one. Sum valid calibrations {}. Time: {:?} \n",
        sum_valid_calibrations, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let calibrations = read_data("./src/history_part_one_data.txt");
    let sum_valid_calibrations = sum_result_valid_calibrations(calibrations, false);
    let duration = start.elapsed();
    print!(
        "History part one. Sum valid calibrations {}. Time: {:?} \n",
        sum_valid_calibrations, duration
    )
}

fn history_example_part_two() {
    let start = Instant::now();
    let calibrations = read_data("./src/history_example_part_two_data.txt");
    let sum_valid_calibrations = sum_result_valid_calibrations(calibrations, true);
    let duration = start.elapsed();
    print!(
        "History example part two. Sum valid calibrations {}. Time: {:?} \n",
        sum_valid_calibrations, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let calibrations = read_data("./src/history_part_two_data.txt");
    let sum_valid_calibrations = sum_result_valid_calibrations(calibrations, true);
    let duration = start.elapsed();
    print!(
        "History part two. Sum valid calibrations {}. Time: {:?} \n",
        sum_valid_calibrations, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
