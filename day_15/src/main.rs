use std::{fs, time::Instant};

struct Coordinate {
    y: usize,
    x: usize,
}

struct Robot {
    map: Vec<Vec<String>>,
    position: Coordinate,
    steps: Vec<String>,
}

impl Robot {
    fn is_wall(&self, y: usize, x: usize) -> bool {
        return self.map[y][x] == "#";
    }

    fn is_free(&self, y: usize, x: usize) -> bool {
        return self.map[y][x] == ".";
    }

    fn is_box(&self, y: usize, x: usize) -> bool {
        return self.map[y][x] == "O";
    }

    // fn print(&self) {
    //     for (y, x_values) in self.map.iter().enumerate() {
    //         for (x, v) in x_values.iter().enumerate() {
    //             if self.position.y == y && self.position.x == x {
    //                 print!("@");
    //             } else {
    //                 print!("{}", v);
    //             }
    //         }
    //         println!("");
    //     }
    // }

    fn move_robot(&mut self) {
        let max_y = self.map.len();
        let max_x = self.map[0].len();

        while self.steps.len() > 0 {
            let direction: &str = &self.steps.remove(0);
            // self.print();
            // println!("Direction {}", direction);

            match direction {
                ">" => {
                    let x = self.position.x + 1;

                    if x > max_x || self.is_wall(self.position.y, x) {
                        continue;
                    }

                    if self.is_free(self.position.y, x) {
                        self.position.x = x;
                        continue;
                    }

                    let mut x_box = x;
                    let mut num_boxes = 0;

                    while x_box + 1 < max_x {
                        if self.is_box(self.position.y, x_box) {
                            x_box += 1;
                            num_boxes += 1;
                        } else {
                            break;
                        }
                    }

                    let mut free_spaces = 0;
                    let x_free = x_box;

                    if num_boxes > 0 && x_free < max_x && self.is_free(self.position.y, x_free){
                        free_spaces += 1;
                    }

                    if free_spaces > 0 {
                        self.position.x = x;

                        for x_move in x..x_free + 1 {
                            if x_move == x {
                                self.map[self.position.y][x_move] = ".".to_string();
                            } else {
                                self.map[self.position.y][x_move] = "O".to_string();
                            }
                        }
                    }
                }
                "<" => {
                    let x: isize = (self.position.x - 1).try_into().unwrap();

                    if x < 0 {
                        continue;
                    }

                    if self.is_wall(self.position.y, x.try_into().unwrap()) {
                        continue;
                    }

                    if self.is_free(self.position.y, x.try_into().unwrap()) {
                        self.position.x = x as usize;
                        continue;
                    }

                    let mut x_box: isize = x as isize;
                    let mut num_boxes = 0;

                    while x_box - 1 >= 0 {
                        if self.is_box(self.position.y, x_box as usize) {
                            x_box -= 1;
                            num_boxes += 1;
                        } else {
                            break;
                        }
                    }

                    let mut free_spaces = 0;
                    let x_free = x_box;

                    if num_boxes > 0 && x_free >= 0 && self.is_free(self.position.y, x_free.try_into().unwrap()){
                        free_spaces += 1;
                    }

                    if free_spaces > 0 {
                        self.position.x = x as usize;

                        let mut x_move = x as isize;

                        while x_move > x_free - 1 {
                            if x_move == x.try_into().unwrap() {
                                self.map[self.position.y][x_move as usize] = ".".to_string();
                            } else {
                                self.map[self.position.y][x_move as usize] = "O".to_string();
                            }
                            x_move -= 1;
                        }
                    }
                }
                "^" => {
                    let y: isize = (self.position.y - 1).try_into().unwrap();

                    if y < 0 {
                        continue;
                    }

                    if self.is_free(y as usize, self.position.x) {
                        self.position.y = y as usize;
                        continue;
                    }

                    let mut y_box = y;
                    let mut num_boxes = 0;

                    while y_box - 1 >= 0 {
                        if self.is_box(y_box as usize, self.position.x) {
                            y_box -= 1;
                            num_boxes += 1;
                        } else {
                            break;
                        }
                    }

                    let mut free_spaces = 0;
                    let y_free = y_box;

                    if num_boxes > 0 && y_free >= 0 &&  self.is_free(y_free as usize, self.position.x){
                        free_spaces += 1;
                    }

                    if free_spaces > 0 {
                        self.position.y = y as usize;

                        let mut y_move = y as isize;

                        while y_move > y_free - 1 {
                            if y_move == y.try_into().unwrap() {
                                self.map[y_move as usize][self.position.x] = ".".to_string();
                            } else {
                                self.map[y_move as usize][self.position.x] = "O".to_string();
                            }
                            y_move -= 1;
                        }
                    }
                }
                "v" => {
                    let y = self.position.y + 1;

                    if y > max_y || self.is_wall(y, self.position.x) {
                        continue;
                    }

                    if self.is_free(y, self.position.x) {
                        self.position.y = y;
                        continue;
                    }

                    let mut y_box = y;
                    let mut num_boxes = 0;

                    while y_box + 1 < max_y {
                        if self.is_box(y_box, self.position.x) {
                            y_box += 1;
                            num_boxes += 1;
                        } else {
                            break;
                        }
                    }

                    let mut free_spaces = 0;
                    let y_free = y_box;

                    if num_boxes > 0 && y_free < max_y && self.is_free(y_free as usize, self.position.x){
                        free_spaces += 1;
                    }

                    if free_spaces > 0 {
                        self.position.y = y;

                        for y_move in y..y_free + 1 {
                            if y_move == y {
                                self.map[y_move][self.position.x] = ".".to_string();
                            } else {
                                self.map[y_move][self.position.x] = "O".to_string();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        //self.print();
    }

    fn calculate_boxes(self) -> usize {
        let mut total: usize = 0;
        for (y, values) in self.map.iter().enumerate() {
            for (x, v) in values.iter().enumerate() {
                if v == "O" {
                    total += 100 * y + x;
                }
            }
        }
        return total;
    }
}

fn read(input_file_name: &str) -> Robot {
    let mut map: Vec<Vec<String>> = Vec::new();
    let mut position: Coordinate = Coordinate { y: 0, x: 0 };
    let content = fs::read_to_string(input_file_name).unwrap();
    let content_split: Vec<&str> = content.split("||").collect();
    let map_coordinates: Vec<&str> = content_split[0].trim().split("\n").collect();

    for (y, value) in map_coordinates.iter().enumerate() {
        let mut values: Vec<String> = Vec::new();

        for (x, v) in value.chars().enumerate() {
            if v == '@' {
                values.push(".".to_string());
                position.y = y;
                position.x = x;
            } else {
                values.push(v.to_string());
            }
        }

        map.push(values);
    }

    let mut steps: Vec<String> = Vec::new();
    let steps_split: Vec<&str> = content_split[1].trim().split("\n").collect();

    for step in steps_split {
        for v in step.chars() {
            let v_str = v.to_string();
            if v_str.trim() != "" {
                steps.push(v_str);
            }
        }
    }

    return Robot {
        map: map,
        position: position,
        steps: steps,
    };
}

fn history_example_part_one() {
    let start = Instant::now();
    let mut robot = read("./src/history_example_part_one_data.txt");
    //let mut robot = read("./src/history_example_part_one_data_slim.txt");
    robot.move_robot();
    let total = robot.calculate_boxes();
    let duration: std::time::Duration = start.elapsed();
    print!(
        "History example part one. Total {}. Time: {:?} \n",
        total, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let mut robot = read("./src/history_part_one_data.txt");
    robot.move_robot();
    let total = robot.calculate_boxes();
    let duration: std::time::Duration = start.elapsed();
    print!(
        "History part one. Total {}. Time: {:?} \n",
        total, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
}
