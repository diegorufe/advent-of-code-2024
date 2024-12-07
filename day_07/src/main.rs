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

    fn validate_operations(&self, index: usize, accumulate: i64) -> bool {
        let mut accumulate_operations_sum: i64 = accumulate;
        let mut accumulate_operations_mult: i64 = accumulate;
        let valid_sum;
        let valid_mult;

        if index == 0 {
            accumulate_operations_sum = self.operands[index];
            accumulate_operations_mult = self.operands[index];

            valid_sum = self.validate_operations(index + 1, accumulate_operations_sum);
            valid_mult = self.validate_operations(index + 1, accumulate_operations_mult);
        } else if index < self.operands.len() {
            accumulate_operations_sum += self.operands[index];
            accumulate_operations_mult *= self.operands[index];

            valid_sum = self.validate_operations(index + 1, accumulate_operations_sum);
            valid_mult = self.validate_operations(index + 1, accumulate_operations_mult);
        } else {
            valid_sum = self.result.eq(&accumulate_operations_sum);
            valid_mult = self.result.eq(&accumulate_operations_mult);
        }

        return valid_sum || valid_mult;
    }

    fn validate(&self) -> bool {
        return self.validate_operations(0, 0);
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

fn sum_result_valid_calibrations(calibrations: Vec<Calibration>) -> i64 {
    let mut sum_valid_calibrations: i64 = 0;

    for calibration in calibrations {
        if calibration.validate() {
            sum_valid_calibrations += calibration.result;
        }
    }

    return sum_valid_calibrations;
}

fn history_example_part_one() {
    let start = Instant::now();
    let calibrations = read_data("./src/history_example_part_one_data.txt");
    let sum_valid_calibrations = sum_result_valid_calibrations(calibrations);
    let duration = start.elapsed();
    print!(
        "History example part one. Sum valid calibrations {}. Time: {:?} \n",
        sum_valid_calibrations, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let calibrations = read_data("./src/history_part_one_data.txt");
    let sum_valid_calibrations = sum_result_valid_calibrations(calibrations);
    let duration = start.elapsed();
    print!(
        "History part one. Sum valid calibrations {}. Time: {:?} \n",
        sum_valid_calibrations, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
}
