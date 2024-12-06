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
    x: isize,
    y: isize,
    map: Vec<Vec<String>>,
    map_visited: Vec<Vec<bool>>,
    x_last: isize,
    y_last: isize,
}

impl Vigilant {
    fn new() -> Vigilant {
        return Vigilant {
            x: 0,
            y: 0,
            direction: Direction::Up,
            map: Vec::new(),
            map_visited: Vec::new(),
            x_last: 0,
            y_last: 0,
        };
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

            if !value.eq("#") {
                moved = true;
                self.x = x_move;
                self.y = y_move;
                self.direction = direction;
                self.map_visited[y_move as usize][x_move as usize] = true;
                self.x_last = self.x;
                self.y_last = self.y;
            }
        }

        return moved;
    }

    fn patrol(&mut self) -> i32 {
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

        for (x, char) in data_lie.chars().enumerate() {
            match char {
                '^' => {
                    found = true;
                    vigilant.direction = Direction::Up;
                }
                '>' => {
                    found = true;
                    vigilant.direction = Direction::Right;
                }
                '<' => {
                    found = true;
                    vigilant.direction = Direction::Left;
                }
                'v' => {
                    found = true;
                    vigilant.direction = Direction::Down;
                }
                _ => {}
            }

            if found {
                vigilant.x = x as isize;
                vigilant.y = y;
                x_values.push(".".to_string());
                found = false;
                x_values_visited.push(true);
            } else {
                x_values.push(char.to_string());
                x_values_visited.push(false);
            }
        }

        y += 1;
        vigilant.map.push(x_values);
        vigilant.map_visited.push(x_values_visited);
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

fn main() {
    history_example_part_one();
    history_part_one();
}
