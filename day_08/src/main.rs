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

    fn calculate_antinodes(&self) -> HashSet<Coordinate> {
        let mut calculated_antinodes: HashSet<Coordinate> = HashSet::new();
        let mut unique_coordinates: HashSet<Coordinate> = HashSet::new();

        for coordinate in self.coordinates.iter() {
            unique_coordinates.insert(*coordinate);
        }

        for (index, coordinate) in self.coordinates.iter().enumerate() {
            let mut next = 0;

            while next < self.coordinates.len() {
                if index == next {
                    next += 1;
                    continue;
                }

                let backguard = self.calculate_antinodes_backguard(
                    coordinate.clone(),
                    self.coordinates[next].clone(),
                );

                if !unique_coordinates.contains(&backguard) && backguard.is_valid() {
                    calculated_antinodes.insert(backguard);
                }

                let forward = self.calculate_antinodes_forward(
                    coordinate.clone(),
                    self.coordinates[next].clone(),
                );

                if !unique_coordinates.contains(&forward) && forward.is_valid() {
                    calculated_antinodes.insert(forward);
                }

                next += 1;
            }
        }

        return calculated_antinodes;
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

fn count_antinodes(antennas: Vec<Antenna>) -> usize {
    let mut calculated_antinodes: HashSet<Coordinate> = HashSet::new();

    for antenna in antennas {
        let calculated_antinodes_antenna = antenna.calculate_antinodes();

        for coordinate in calculated_antinodes_antenna {
            calculated_antinodes.insert(coordinate);
        }
    }

    return calculated_antinodes.len();
}

fn history_example_part_one() {
    let start = Instant::now();
    let anntenas: Vec<Antenna> = read_data("./src/history_example_part_one_data.txt");
    let count_antinodes = count_antinodes(anntenas);
    let duration = start.elapsed();
    print!(
        "History example part one. Count antinodes {}. Time: {:?} \n",
        count_antinodes, duration
    )
}

fn history_part_one() {
    let start = Instant::now();
    let anntenas: Vec<Antenna> = read_data("./src/history_part_one_data.txt");
    let count_antinodes = count_antinodes(anntenas);
    let duration = start.elapsed();
    print!(
        "History part one. Count antinodes {}. Time: {:?} \n",
        count_antinodes, duration
    )
}

fn main() {
    history_example_part_one();
    history_part_one();
}
