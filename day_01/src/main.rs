use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn convert_to_vectors(input_file_name: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vect_input_a: Vec<i32> = Vec::new();
    let mut vect_input_b: Vec<i32> = Vec::new();

    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let values: Vec<i32> = line
            .expect("Error read line")
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        vect_input_a.push(values[0]);
        vect_input_b.push(values[1]);
    }

    vect_input_a.sort();
    vect_input_b.sort();
    return (vect_input_a, vect_input_b);
}

fn calculate_distance(location_ids_a: Vec<i32>, location_ids_b: Vec<i32>) -> i32 {
    if location_ids_a.len() != location_ids_b.len() {
        panic!("Locations size is not the same");
    }

    let mut location_ids_d: Vec<i32> = Vec::new();
    for i in 0..location_ids_a.len() {
        location_ids_d.push((location_ids_a[i] - location_ids_b[i]).abs());
    }

    return location_ids_d.iter().sum();
}

fn calculate_score(location_ids_a: Vec<i32>, location_ids_b: Vec<i32>) -> i32 {
    let mut location_ids_d: Vec<i32> = Vec::new();

    for value in location_ids_a.iter() {
        let count: i32 = location_ids_b.iter().filter(|&s| s == value).count() as i32;
        location_ids_d.push(value * count);
    }

    return location_ids_d.iter().sum();
}

fn history_example_part_one() {
    let start = Instant::now();
    let (location_ids_a, location_ids_b): (Vec<i32>, Vec<i32>) =
        convert_to_vectors("./src/history_example_part_one_data.txt");
    let distance = calculate_distance(location_ids_a, location_ids_b);
    let duration = start.elapsed();
    print!(
        "History example part one. Distance {}. Time: {:?} \n",
        distance, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let (location_ids_a, location_ids_b): (Vec<i32>, Vec<i32>) =
        convert_to_vectors("./src/history_part_one_data.txt");
    let distance = calculate_distance(location_ids_a, location_ids_b);
    let duration = start.elapsed();
    print!(
        "History part one. Distance {}. Time: {:?}  \n",
        distance, duration
    )
}

fn history_example_part_two() {
    let start = Instant::now();
    let (location_ids_a, location_ids_b): (Vec<i32>, Vec<i32>) =
        convert_to_vectors("./src/history_example_part_two_data.txt");
    let score = calculate_score(location_ids_a, location_ids_b);
    let duration = start.elapsed();
    print!(
        "History example part two. Score {}. Time: {:?} \n",
        score, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let (location_ids_a, location_ids_b): (Vec<i32>, Vec<i32>) =
        convert_to_vectors("./src/history_part_two_data.txt");
    let score = calculate_score(location_ids_a, location_ids_b);
    let duration = start.elapsed();
    print!("History part two. Score {}. Time: {:?} \n", score, duration)
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
