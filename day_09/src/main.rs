use std::{fs, time::Instant};

#[derive(Clone, Copy)]
struct Block {
    size: i32,
    id: isize,
}

fn read_data(input_file_name: &str) -> String {
    let content = fs::read_to_string(input_file_name).unwrap();
    return content;
}

fn compact_disk(data: String) -> i64 {
    let mut id_disk: isize = 0;
    let mut disk: Vec<isize> = Vec::new();

    for (index, value) in data.chars().enumerate() {
        let size_repeat = value.to_digit(10).unwrap().try_into().unwrap();

        if index % 2 == 0 {
            for _ in 0..size_repeat {
                disk.push(id_disk);
            }
            id_disk += 1;
        } else {
            if size_repeat > 0 {
                for _ in 0..size_repeat {
                    disk.push(-1);
                }
            }
        }
    }

    let mut last_index = disk.len() - 1;

    for (index, data) in disk.clone().iter().enumerate() {
        if index >= last_index {
            break;
        }

        if *data > -1 {
            continue;
        }

        let mut last_index_find = last_index;

        while last_index_find > index + 1 {
            let data_find = disk[last_index_find];

            if data_find == -1 {
                last_index_find -= 1;
            } else {
                last_index = last_index_find;
                disk[index] = disk[last_index_find];
                disk[last_index_find] = -1;

                if last_index_find == last_index {
                    last_index -= 1
                }

                break;
            }
        }
    }

    let mut result: i64 = 0;

    for (index, data) in disk.iter().enumerate() {
        if *data == -1 {
            continue;
        }
        result += index as i64 * *data as i64;
    }

    return result;
}

fn compact_disk_block(data: String) -> i64 {
    let mut id_disk: isize = 0;
    let mut disk: Vec<Block> = Vec::new();

    for (index, value) in data.chars().enumerate() {
        let size_repeat = value.to_digit(10).unwrap().try_into().unwrap();

        if index % 2 == 0 {
            disk.push(Block {
                size: size_repeat,
                id: id_disk,
            });
            id_disk += 1;
        } else {
            if size_repeat > 0 {
                disk.push(Block {
                    size: size_repeat,
                    id: -1,
                });
            }
        }
    }

    let mut last_index = disk.len() - 1;

    while last_index > 0 {
        let block = disk[last_index];

        if block.id == -1 {
            last_index -= 1;
            continue;
        }

        for index_free in 0..last_index {
            let block_free = disk[index_free];

            if block_free.id == -1 && block_free.size >= block.size {
                disk[index_free] = block.clone();

                let mut block_free_size = block.clone();
                block_free_size.id = -1;

                if block_free.size - block.size > 0 {
                    let result_size = block_free.size - block.size;
                    block_free_size.size = block.size;
                    disk[last_index] = block_free_size;
                    let mut block_sub_size = block_free.clone();
                    block_sub_size.size = result_size;
                    disk.insert(index_free + 1, block_sub_size);
                    last_index += 1;
                } else {
                    disk[last_index] = block_free_size;
                }
                break;
            }
        }

        last_index -= 1;
    }

    let mut result: i64 = 0;
    let mut block_index = 0;

    for data in disk.iter() {
        if data.id != -1 {
            for i in 0..data.size {
                result += (block_index as i32 + i) as i64 * data.id as i64;
            }
        }

        block_index += data.size as usize;
    }

    return result;
}

fn history_example_part_one() {
    let start = Instant::now();
    let data: String = read_data("./src/history_example_part_one_data.txt");
    let result = compact_disk(data);
    let duration = start.elapsed();
    print!(
        "History example part one. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let data: String = read_data("./src/history_part_one_data.txt");
    let result = compact_disk(data);
    let duration = start.elapsed();
    print!(
        "History part one. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn history_example_part_two() {
    let start = Instant::now();
    let data: String = read_data("./src/history_example_part_two_data.txt");
    let result = compact_disk_block(data);
    let duration = start.elapsed();
    print!(
        "History example part two. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let data: String = read_data("./src/history_part_two_data.txt");
    let result = compact_disk_block(data);
    let duration = start.elapsed();
    print!(
        "History part two. Result {}. Time: {:?} \n",
        result, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
