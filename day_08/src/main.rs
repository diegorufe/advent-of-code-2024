use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
struct Coordinate {
    y: isize,
    x: isize,
    max_y: isize,
    max_x: isize,
}

impl Coordinate {
    fn new(y: isize, x: isize, max_y: isize, max_x: isize) -> Self {
        return Self {
            y: y,
            x: x,
            max_y: max_y,
            max_x: max_x,
        };
    }

    fn is_valid(&self) -> bool {
        return self.x >= 0 && self.y >= 0 && self.x < self.max_x && self.y < self.max_y;
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Antenna {
    key: String,
    coordinates: Vec<Coordinate>,
}

impl Antenna {
    fn new(key: String) -> Self {
        return Self {
            key: key,
            coordinates: Vec::new(),
        };
    }

    fn push_coordinate(&mut self, y: i32, x: i32, max_y: usize, max_x: usize) {
        self.coordinates.push(Coordinate::new(
            y as isize,
            x as isize,
            max_y as isize,
            max_x as isize,
        ));
    }

    fn calculate_antinodes_backguard(&self, first: Coordinate, second: Coordinate) -> Coordinate {
        let y = first.y - (first.y - second.y);
        let x = first.x - (first.x - second.x);
        return Coordinate::new(y, x, first.max_y, first.max_x);
    }

    fn calculate_antinodes_forward(&self, first: Coordinate, second: Coordinate) -> Coordinate {
        let y = second.y + ((first.y - second.y) * -1);
        let x = second.x + ((first.x - second.x) * -1);
        return Coordinate::new(y, x, first.max_y, first.max_x);
    }

    fn calculate_t_antinodes_backguard(
        &self,
        first: Coordinate,
        second: Coordinate,
    ) -> Vec<Coordinate> {
        if second.y <= first.y {
            return Vec::new();
        }

        let mut values: Vec<Coordinate> = Vec::new();

        let mut index_calculate = first.y;

        let y_diff = (second.y - first.y) * -1;
        let x_diff = (second.x - first.x) * -1;

        let mut y = first.y;
        let mut x = first.x;

        while index_calculate > 0 {
            y += y_diff;
            x += x_diff;

            values.push(Coordinate::new(y, x, first.max_y, first.max_x));

            index_calculate = y;
        }

        return values;
    }

    fn calculate_t_antinodes_forward(
        &self,
        first: Coordinate,
        second: Coordinate,
    ) -> Vec<Coordinate> {
        if first.y >= second.y {
            return Vec::new();
        }

        let size = first.max_y as isize;
        let mut values: Vec<Coordinate> = Vec::new();

        let mut index_calculate = first.y;

        let y_diff = second.y - first.y;
        let x_diff = second.x - first.x;

        let mut y = first.y;
        let mut x = first.x;

        while index_calculate < size {
            y += y_diff;
            x += x_diff;

            values.push(Coordinate::new(y, x, first.max_y, first.max_x));

            index_calculate = y;
        }

        return values;
    }

    fn calculate_antinodes(
        &self,
        calculate_t_antinodes: bool,
    ) -> (HashSet<Coordinate>, HashSet<Coordinate>) {
        let mut calculated_antinodes: HashSet<Coordinate> = HashSet::new();
        let mut unique_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut calculated_t_antinodes: HashSet<Coordinate> = HashSet::new();

        for coordinate in self.coordinates.iter() {
            unique_coordinates.insert(*coordinate);
        }

        for (index, coordinate) in self.coordinates.iter().enumerate() {
            let mut next = 0;

            while next < self.coordinates.len() {
                let first = coordinate.clone();
                let second = self.coordinates[next].clone();

                if calculate_t_antinodes {
                    let t_backguard_values: Vec<Coordinate> =
                        self.calculate_t_antinodes_backguard(first, second);

                    for t_backguard in t_backguard_values {
                        if t_backguard.is_valid() {
                            calculated_t_antinodes.insert(t_backguard);
                        }
                    }

                    let t_forward_values: Vec<Coordinate> =
                        self.calculate_t_antinodes_forward(first, second);

                    for t_forward in t_forward_values {
                        if t_forward.is_valid() {
                            calculated_t_antinodes.insert(t_forward);
                        }
                    }
                }

                if index == next {
                    next += 1;
                    continue;
                }

                let backguard = self.calculate_antinodes_backguard(first, second);

                if !unique_coordinates.contains(&backguard) && backguard.is_valid() {
                    calculated_antinodes.insert(backguard);
                }

                let forward = self.calculate_antinodes_forward(first, second);

                if !unique_coordinates.contains(&forward) && forward.is_valid() {
                    calculated_antinodes.insert(forward);
                }

                next += 1;
            }
        }

        return (calculated_antinodes, calculated_t_antinodes);
    }
}

fn read_data(input_file_name: &str) -> Vec<Antenna> {
    let mut map_antennas: HashMap<String, Antenna> = HashMap::new();
    let mut file = File::open(input_file_name).expect("Unable to read file");
    let mut reader = io::BufReader::new(file);
    let mut y = 0;
    let max_y = reader.lines().count();
    file = File::open(input_file_name).expect("Unable to read file");
    reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line_value = line.expect("Unable to read line");

        for (x, value) in line_value.chars().enumerate() {
            if value == '.' || value == '#' {
                continue;
            }

            map_antennas
                .entry(value.to_string())
                .or_insert(Antenna::new(value.to_string()))
                .push_coordinate(y, x as i32, max_y, line_value.len());
        }

        y += 1;
    }

    let mut antennas: Vec<Antenna> = Vec::new();

    for antenna in map_antennas.values() {
        antennas.push(antenna.clone());
    }

    return antennas;
}

fn count_antinodes(antennas: Vec<Antenna>, calculate_t_antinodes: bool) -> usize {
    let mut calculated_antinodes: HashSet<Coordinate> = HashSet::new();

    for antenna in antennas {
        let (calculated_antinodes_antenna, calculated_t_antinodes_antenna) =
            antenna.calculate_antinodes(calculate_t_antinodes);

        for coordinate in calculated_antinodes_antenna {
            calculated_antinodes.insert(coordinate);
        }

        for coordinate in calculated_t_antinodes_antenna {
            calculated_antinodes.insert(coordinate);
        }

        if calculate_t_antinodes {
            for coordinate in antenna.coordinates {
                calculated_antinodes.insert(coordinate);
            }
        }
    }

    return calculated_antinodes.len();
}

fn history_example_part_one() {
    let start = Instant::now();
    let anntenas: Vec<Antenna> = read_data("./src/history_example_part_one_data.txt");
    let count_antinodes = count_antinodes(anntenas, false);
    let duration = start.elapsed();
    print!(
        "History example part one. Count antinodes {}. Time: {:?} \n",
        count_antinodes, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let anntenas: Vec<Antenna> = read_data("./src/history_part_one_data.txt");
    let count_antinodes = count_antinodes(anntenas, false);
    let duration = start.elapsed();
    print!(
        "History part one. Count antinodes {}. Time: {:?} \n",
        count_antinodes, duration
    )
}

fn history_example_part_two() {
    let start = Instant::now();
    let anntenas: Vec<Antenna> = read_data("./src/history_example_part_two_data.txt");
    let count_antinodes = count_antinodes(anntenas, true);
    let duration = start.elapsed();
    print!(
        "History example part two. Count antinodes {}. Time: {:?} \n",
        count_antinodes, duration
    )
}

fn history_part_two() {
    let start = Instant::now();
    let anntenas: Vec<Antenna> = read_data("./src/history_part_two_data.txt");
    let count_antinodes = count_antinodes(anntenas, true);
    let duration = start.elapsed();
    print!(
        "History part two. Count antinodes {}. Time: {:?} \n",
        count_antinodes, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
    history_example_part_two();
    history_part_two();
}
