use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

struct Coordinate {
    x: isize,
    y: isize,
}

struct Robot {
    p: Coordinate,
    v: Coordinate,
}

impl Robot {
    fn move_robot(&mut self, x: isize, y: isize) {
        let mut x_move = self.p.x + self.v.x;
        let mut y_move = self.p.y + self.v.y;

        if x_move < 0 {
            x_move = x + 1 - (x_move as i32).abs() as isize;
        }

        if x_move > x {
            x_move = x_move - (x + 1);
        }

        if y_move < 0 {
            y_move = y + 1 - (y_move as i32).abs() as isize;
        }

        if y_move > y {
            y_move = y_move - (y + 1);
        }

        self.p.x = x_move;
        self.p.y = y_move;
    }
}

struct Tree {
    values: Vec<Vec<usize>>,
}

impl Tree {
    fn clear(&mut self) {
        for x_values in self.values.iter_mut() {
            for x in 0..x_values.len() {
                x_values[x] = 0;
            }
        }
    }

    fn add(&mut self, robot: &mut Robot) {
        let mut x_values = self.values[robot.p.y as usize].clone();
        x_values[robot.p.x as usize] = 1;
        self.values[robot.p.y as usize] = x_values;
    }

    fn print(&self) {
        for x_values in self.values.iter() {
            for x in x_values {
                if *x == 0 {
                    print!("-")
                } else {
                    print!("*")
                }
            }
            println!("");
        }
    }

    fn check(&self) -> bool {
        let max_x_values = 6;
        let mut valid = false;

        for (y, x_values) in self.values.iter().enumerate() {
            let mut count = 0;
            let mut min_x = 0;
            let mut max_x = 0;

            for (index, x) in x_values.iter().enumerate() {
                if *x > 0 {
                    if min_x == 0 {
                        min_x = index;
                    }

                    count += 1;

                    if count == max_x_values {
                        max_x = index;
                        break;
                    }
                } else if count < max_x_values {
                    count = 0;
                    min_x = 0;
                    max_x = 0;
                }
            }

            if count == max_x_values {
                let total_before: usize = self.values[y - 1][min_x - 1..max_x + 1].iter().sum();
                let total_after: usize = self.values[y + 1][min_x - 1..max_x + 1].iter().sum();
                if total_before >= 1 && total_after >= max_x_values {
                    valid = true;
                    self.print();
                    break;
                }
            }
        }

        return valid;
    }
}

fn read(input_file_name: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();
    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let values: Vec<String> = line
            .expect("Error read line")
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let p_values: Vec<&str> = values[0].split("=").collect();
        let p_coordinates: Vec<isize> =
            p_values[1].split(",").map(|s| s.parse().unwrap()).collect();

        let v_values: Vec<&str> = values[1].split("=").collect();
        let v_coordinates: Vec<isize> =
            v_values[1].split(",").map(|s| s.parse().unwrap()).collect();

        robots.push(Robot {
            p: Coordinate {
                x: p_coordinates[0],
                y: p_coordinates[1],
            },
            v: Coordinate {
                x: v_coordinates[0],
                y: v_coordinates[1],
            },
        });
    }

    return robots;
}

fn calculate(robots: &mut Vec<Robot>, x: isize, y: isize, seconds: isize) -> usize {
    let x_middle = (x as f64 / 2.0).ceil() as isize;
    let y_middle = (y as f64 / 2.0).ceil() as isize;
    let mut total_quadrant: Vec<usize> = Vec::new();

    for _ in 0..seconds {
        total_quadrant = vec![0, 0, 0, 0];
        for robot in &mut *robots {
            robot.move_robot(x, y);

            if robot.p.x != x_middle && robot.p.y != y_middle {
                if robot.p.x < x_middle && robot.p.y < y_middle {
                    total_quadrant[0] += 1;
                } else if robot.p.x > x_middle && robot.p.y < y_middle {
                    total_quadrant[1] += 1;
                } else if robot.p.x < x_middle && robot.p.y > y_middle {
                    total_quadrant[2] += 1;
                } else {
                    total_quadrant[3] += 1;
                }
            }
        }
    }

    let mut total = 1;

    for t in total_quadrant {
        total *= t;
    }

    return total;
}

fn calculate_tree(robots: &mut Vec<Robot>, x: isize, y: isize) -> usize {
    let mut y_values: Vec<Vec<usize>> = Vec::new();
    for _ in 0..y + 1 {
        let mut x_values: Vec<usize> = Vec::new();
        for _ in 0..x + 1 {
            x_values.push(0);
        }
        y_values.push(x_values);
    }

    let mut tree: Tree = Tree { values: y_values };

    let max_seconds = 100000;
    let mut second = 1;
    let mut tree_checked = false;

    while second < max_seconds && !tree_checked {
        tree.clear();

        for robot in &mut *robots {
            robot.move_robot(x, y);
            tree.add(robot);
        }

        tree_checked = tree.check();
        second += 1;
    }

    return second - 1;
}

fn history_example_part_one() {
    let start = Instant::now();
    let mut robots = read("./src/history_example_part_one_data.txt");
    let total = calculate(&mut robots, 10, 6, 100);
    let duration: std::time::Duration = start.elapsed();
    print!(
        "History example part one. Total {}. Time: {:?} \n",
        total, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let mut robots = read("./src/history_part_one_data.txt");
    let total = calculate(&mut robots, 100, 102, 100);
    let duration: std::time::Duration = start.elapsed();
    print!("History part one. Total {}. Time: {:?} \n", total, duration)
}

fn history_part_two() {
    let start = Instant::now();
    let mut robots = read("./src/history_part_two_data.txt");
    let total = calculate_tree(&mut robots, 100, 102);
    let duration: std::time::Duration = start.elapsed();
    print!("History part two. Total {}. Time: {:?} \n", total, duration)
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_part_two();
}
