use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Vigilant {
    direction: Direction,
    direction_start: Direction,
    x: isize,
    y: isize,
    map: Vec<Vec<String>>,
    map_visited: Vec<Vec<bool>>,
    x_last: isize,
    y_last: isize,
    x_start: isize,
    y_start: isize,
    overflow: bool,
    overflow_visisted: Vec<Vec<bool>>,
    count_visited: Vec<Vec<i32>>,
    possible_overflow: bool,
}

impl Vigilant {
    fn new() -> Vigilant {
        return Vigilant {
            x: 0,
            y: 0,
            direction: Direction::Up,
            direction_start: Direction::Up,
            map: Vec::new(),
            map_visited: Vec::new(),
            x_last: 0,
            y_last: 0,
            x_start: 0,
            y_start: 0,
            overflow: false,
            overflow_visisted: Vec::new(),
            count_visited: Vec::new(),
            possible_overflow: false,
        };
    }

    fn clear(&mut self) {
        self.x = self.x_start;
        self.y = self.y_start;
        self.x_last = -1;
        self.y_last = -1;
        self.overflow = false;
        self.direction = self.direction_start.clone();
        self.possible_overflow = false;

        for y in 0..self.overflow_visisted.len() {
            for x in 0..self.overflow_visisted[0].len() {
                self.overflow_visisted[y][x] = false;
                self.count_visited[y][x] = 0;
            }
        }
    }

    fn move_direction(&mut self, direction: Direction) -> bool {
        let max_x: isize = (self.map[0].len() - 1) as isize;
        let max_y: isize = (self.map.len() - 1) as isize;
        let mut moved = false;
        let mut y_move = self.y;
        let mut x_move = self.x;

        match direction {
            Direction::Up => {
                y_move -= 1;
            }
            Direction::Right => {
                x_move += 1;
            }
            Direction::Left => {
                x_move -= 1;
            }
            Direction::Down => {
                y_move += 1;
            }
        }

        if x_move < 0 || x_move > max_x || y_move < 0 || y_move > max_y {
            self.x = -1;
            self.y = -1;
        } else {
            let value = &self.map[y_move as usize][x_move as usize];

            if value.eq("0") {
                self.possible_overflow = true;

                match direction {
                    Direction::Up => {
                        y_move += 1;
                    }
                    Direction::Right => {
                        x_move -= 1;
                    }
                    Direction::Left => {
                        x_move += 1;
                    }
                    Direction::Down => {
                        y_move -= 1;
                    }
                }

                if self.overflow_visisted[y_move as usize][x_move as usize] {
                    self.overflow = true;
                    self.x = -1;
                    self.y = -1;
                }

                self.overflow_visisted[y_move as usize][x_move as usize] = true;
            } else if !value.eq("#") {
                moved = true;
                self.x = x_move;
                self.y = y_move;
                self.direction = direction;
                self.map_visited[y_move as usize][x_move as usize] = true;
                self.x_last = self.x;
                self.y_last = self.y;

                if self.possible_overflow {
                    self.count_visited[self.y as usize][self.x as usize] += 1;

                    if self.count_visited[self.y as usize][self.x as usize] > 30 {
                        self.overflow = true;
                        self.x = -1;
                        self.y = -1;
                    }
                }
            }
        }

        return moved;
    }

    fn patrol(&mut self) -> i32 {
        self.clear();
        let map_directions = generate_map_directions();

        while self.x >= 0 || self.y >= 0 {
            let directions = map_directions
                .get(&self.direction)
                .expect("unable to find directions");
            let mut moved = false;

            while !moved {
                for direction in directions {
                    moved = self.move_direction(direction.clone());

                    if moved {
                        break;
                    }

                    if self.x < 0 || self.y < 0 {
                        moved = true;
                        break;
                    }
                }
            }
        }

        let mut visited_positions = 0;

        for values in self.map_visited.clone() {
            for visited in values {
                if visited {
                    visited_positions += 1;
                }
            }
        }

        return visited_positions;
    }
}

fn generate_map_directions() -> HashMap<Direction, Vec<Direction>> {
    let mut map_directions: HashMap<Direction, Vec<Direction>> = HashMap::new();
    map_directions.insert(
        Direction::Up,
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ],
    );
    map_directions.insert(
        Direction::Right,
        vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ],
    );
    map_directions.insert(
        Direction::Left,
        vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ],
    );
    map_directions.insert(
        Direction::Down,
        vec![
            Direction::Down,
            Direction::Left,
            Direction::Up,
            Direction::Right,
        ],
    );

    return map_directions;
}

fn read_data(input_file_name: &str) -> Vigilant {
    let file = File::open(input_file_name).expect("Unable to read file");
    let reader = io::BufReader::new(file);
    let mut vigilant: Vigilant = Vigilant::new();
    let mut y = 0;

    for line in reader.lines() {
        let data_lie = line.expect("Unable to read line");
        let mut found = false;
        let mut x_values: Vec<String> = Vec::new();
        let mut x_values_visited: Vec<bool> = Vec::new();
        let mut x_overflow_visited: Vec<bool> = Vec::new();
        let mut x_count_visited: Vec<i32> = Vec::new();

        for (x, char) in data_lie.chars().enumerate() {
            match char {
                '^' => {
                    found = true;
                    vigilant.direction = Direction::Up;
                    vigilant.direction_start = Direction::Up;
                }
                '>' => {
                    found = true;
                    vigilant.direction = Direction::Right;
                    vigilant.direction_start = Direction::Right;
                }
                '<' => {
                    found = true;
                    vigilant.direction = Direction::Left;
                    vigilant.direction_start = Direction::Left;
                }
                'v' => {
                    found = true;
                    vigilant.direction = Direction::Down;
                    vigilant.direction_start = Direction::Down;
                }
                _ => {}
            }

            if found {
                vigilant.x = x as isize;
                vigilant.y = y;
                x_values.push(".".to_string());
                found = false;
                x_values_visited.push(true);
                vigilant.x_start = vigilant.x;
                vigilant.y_start = vigilant.y;
            } else {
                x_values.push(char.to_string());
                x_values_visited.push(false);
            }

            x_overflow_visited.push(false);
            x_count_visited.push(0);
        }

        y += 1;
        vigilant.map.push(x_values);
        vigilant.map_visited.push(x_values_visited);
        vigilant.overflow_visisted.push(x_overflow_visited);
        vigilant.count_visited.push(x_count_visited);
    }

    return vigilant;
}

fn history_example_part_one() {
    let start = Instant::now();
    let mut vigilant = read_data("./src/history_example_part_one_data.txt");
    let positions_visited = vigilant.patrol();
    let duration = start.elapsed();
    print!(
        "History example part one. Distinct position visited {}. Time: {:?} \n",
        positions_visited, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let mut vigilant = read_data("./src/history_part_one_data.txt");
    let positions_visited = vigilant.patrol();
    let duration = start.elapsed();
    print!(
        "History part one. Distinct position visited {}. Time: {:?} \n",
        positions_visited, duration
    )
}

fn overflow_partrol(mut vigilant: Vigilant) -> i32 {
    let mut count_overflow_rutes = 0;
    let map_routes_vigilant = vigilant.map.clone();
    let x_size = vigilant.map[0].len();
    let y_size = vigilant.map.len();

    for y in 0..y_size {
        for x in 0..x_size {
            // println!("Y {}, X {}", y, x);

            let mut map_routes = map_routes_vigilant.clone();

            if (y == vigilant.y_start.try_into().unwrap()
                && x == vigilant.x_start.try_into().unwrap())
                || map_routes[y][x].eq("#")
            {
                continue;
            }

            map_routes[y][x] = "0".to_string();

            vigilant.map = map_routes;
            vigilant.patrol();

            if vigilant.overflow {
                count_overflow_rutes += 1;
            }
        }
    }

    return count_overflow_rutes;
}

fn history_example_part_two() {
    let start: Instant = Instant::now();
    let vigilant: Vigilant = read_data("./src/history_example_part_two_data.txt");
    let overflow_routes = overflow_partrol(vigilant);
    let duration = start.elapsed();
    print!(
        "History example part two. Overflow routes {}. Time: {:?} \n",
        overflow_routes, duration
    )
}

fn history_part_two() {
    let start: Instant = Instant::now();
    let vigilant: Vigilant = read_data("./src/history_part_two_data.txt");
    let overflow_routes = overflow_partrol(vigilant);
    let duration = start.elapsed();
    print!(
        "History part two. Overflow routes {}. Time: {:?} \n",
        overflow_routes, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
