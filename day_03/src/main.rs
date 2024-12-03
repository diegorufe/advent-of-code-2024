use std::fs;
use std::time::Instant;

fn read_file(input_file_name: &str) -> String {
    return fs::read_to_string(input_file_name).expect("Unable to read file");
}

fn parse(data: String, check_operation_enabled: bool) -> i32 {
    let do_operation_key: &str = "do()";
    let dont_operation_key: &str = "don't()";

    let mut result: i32 = 0;
    let mut builder_operation = String::new();
    let mut builder_multiplicand = String::new();
    let mut builder_multiplier = String::new();
    let mut builder = String::new();
    let mut enabled_operation: bool = true;

    for character in data.chars() {
        builder.push_str(character.to_string().as_str());

        if check_operation_enabled
            && (builder.contains(do_operation_key) || builder.contains(dont_operation_key))
        {
            enabled_operation = builder.contains(do_operation_key);
            builder.clear();
        }

        match character {
            'm' => {
                resolve_character_in_builder(&mut builder_operation, character, 0);
            }
            'u' => {
                resolve_character_in_builder(&mut builder_operation, character, 1);
            }
            'l' => {
                resolve_character_in_builder(&mut builder_operation, character, 2);
            }
            '(' => {
                resolve_character_in_builder(&mut builder_operation, character, 3);
            }
            ',' => {
                if builder_operation.len() == 4 {
                    builder_operation.push_str(character.to_string().as_str());
                } else {
                    clear_builders(
                        &mut builder_operation,
                        &mut builder_multiplicand,
                        &mut builder_multiplier,
                    );
                }
            }
            ')' => {
                if builder_operation.len() == 5 && enabled_operation {
                    let multiplicand: i32 = builder_multiplicand
                        .parse()
                        .expect("Error on parsing multiplicand");
                    let multiplier: i32 = builder_multiplier
                        .parse()
                        .expect("Error on parsing multiplier");
                    let product: i32 = multiplicand * multiplier;
                    result = result + product;
                }

                clear_builders(
                    &mut builder_operation,
                    &mut builder_multiplicand,
                    &mut builder_multiplier,
                );
            }
            _ => {
                if character.is_digit(10) && builder_operation.len() == 4 {
                    builder_multiplicand.push_str(character.to_string().as_str());
                } else if character.is_digit(10) && builder_operation.len() == 5 {
                    builder_multiplier.push_str(character.to_string().as_str());
                } else {
                    clear_builders(
                        &mut builder_operation,
                        &mut builder_multiplicand,
                        &mut builder_multiplier,
                    );
                }
            }
        }
    }

    return result;
}

fn resolve_character_in_builder(builder: &mut String, character: char, length: i32) {
    if builder.len() == length.try_into().unwrap() {
        builder.push_str(character.to_string().as_str());
    } else {
        builder.clear();
    }
}

fn clear_builders(
    builder: &mut String,
    builder_multiplicand: &mut String,
    builder_multiplier: &mut String,
) {
    builder.clear();
    builder_multiplicand.clear();
    builder_multiplier.clear();
}

fn history_example_part_one() {
    let start = Instant::now();
    let data: String = read_file("./src/history_example_part_one_data.txt");
    let result = parse(data, false);
    let duration = start.elapsed();
    print!(
        "History example part one. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let data = read_file("./src/history_part_one_data.txt");
    let result = parse(data, false);
    let duration = start.elapsed();
    print!(
        "History part one. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn history_example_part_two() {
    let start = Instant::now();
    let data: String = read_file("./src/history_example_part_two_data.txt");
    let result = parse(data, true);
    let duration = start.elapsed();
    print!(
        "History example part one. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let data: String = read_file("./src/history_part_two_data.txt");
    let result = parse(data, true);
    let duration = start.elapsed();
    print!(
        "History example part one. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
