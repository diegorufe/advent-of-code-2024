use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn convert_to_vector(input_file_name: &str) -> Vec<String> {
    let mut vect_input: Vec<String> = Vec::new();

    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        vect_input.push(line.expect("Error when read line"));
    }

    return vect_input;
}

fn count_world(input_values: &Vec<String>) -> i32 {
    let mut count = count_vertical(input_values);
    count = count + count_horizontal(input_values);
    count = count + count_diagonal_right_direction(input_values);
    count = count + count_diagonal_left_direction(input_values);

    return count;
}

fn count_vertical(input_values: &Vec<String>) -> i32 {
    return count_match(input_values);
}

fn count_horizontal(input_values: &Vec<String>) -> i32 {
    let mut horizontal_values: Vec<String> = Vec::new();

    for _ in 0..input_values[0].len() {
        horizontal_values.push(String::new());
    }

    for value in input_values {
        for (index, character) in value.chars().enumerate() {
            horizontal_values[index].push(character);
        }
    }

    return count_match(&horizontal_values);
}

fn count_diagonal_right_direction(input_values: &Vec<String>) -> i32 {
    let mut diagonal_values: Vec<String> = Vec::new();
    let word_size = input_values[0].len();

    for (char_index, character) in input_values[0].chars().enumerate() {
        let builder =
            build_word_index_right_direction(0, char_index, word_size, character, input_values);
        diagonal_values.push(builder);
    }

    for (index, value) in input_values.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let builder = build_word_index_right_direction(
            index,
            0,
            word_size,
            value.chars().next().expect("Unable to find first char"),
            input_values,
        );
        diagonal_values.push(builder);
    }

    return count_match(&diagonal_values);
}

fn build_word_index_right_direction(
    origin_values_index: usize,
    char_index: usize,
    word_size: usize,
    character: char,
    input_values: &Vec<String>,
) -> String {
    let mut builder = String::new();
    builder.push(character);

    let mut word_index: usize = char_index + 1;
    let mut values_index = origin_values_index + 1;

    while word_index < word_size && values_index < input_values.len() {
        let value: &String = &input_values[values_index];
        let word = &value[word_index..word_index + 1];
        builder.push_str(word);
        word_index = word_index + 1;
        values_index = values_index + 1;
    }

    return builder;
}

fn count_diagonal_left_direction(input_values: &Vec<String>) -> i32 {
    let mut diagonal_values: Vec<String> = Vec::new();

    for (char_index, character) in input_values[0].chars().enumerate() {
        let builder = build_word_index_left_direction(0, char_index, character, input_values);
        diagonal_values.push(builder);
    }

    for (index, value) in input_values.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let builder = build_word_index_left_direction(
            index,
            value.len() - 1,
            value.chars().last().expect("Unable to find last char"),
            input_values,
        );
        diagonal_values.push(builder);
    }

    return count_match(&diagonal_values);
}

fn build_word_index_left_direction(
    origin_values_index: usize,
    char_index: usize,
    character: char,
    input_values: &Vec<String>,
) -> String {
    let mut builder = String::new();
    builder.push(character);

    let mut word_index: isize = char_index as isize - 1;
    let mut values_index = origin_values_index + 1;

    while word_index >= 0 && values_index < input_values.len() {
        let value: &String = &input_values[values_index];
        let word = &value[word_index as usize..word_index as usize + 1];
        builder.push_str(word);
        word_index = word_index - 1;
        values_index = values_index + 1;
    }

    return builder;
}

fn count_match(input_values: &Vec<String>) -> i32 {
    let xmas = "XMAS";
    let xmas_reverse = "SAMX";
    let mut count: i32 = 0;

    for value in input_values {
        count = count + count_match_word(xmas, value);
        count = count + count_match_word(xmas_reverse, value);
    }

    return count;
}

fn count_match_word(word: &str, value: &str) -> i32 {
    let mathes: Vec<&str> = value.matches(word).collect();
    return mathes.len() as i32;
}

fn history_example_part_one() {
    let start = Instant::now();
    let input_values = convert_to_vector("./src/history_example_part_one_data.txt");
    let count = count_world(&input_values);
    let duration = start.elapsed();
    print!(
        "History example part one. Count {}. Time: {:?}  \n",
        count, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let input_values = convert_to_vector("./src/history_part_one_data.txt");
    let count = count_world(&input_values);
    let duration = start.elapsed();
    print!(
        "History part one. Count {}. Time: {:?}  \n",
        count, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
}
