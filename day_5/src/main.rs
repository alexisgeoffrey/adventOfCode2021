use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

const FILE_PATH: &str = "C:/Users/Alexis/dev/adventofcode/2021/day_5/src/input.txt";

fn main() {
    println!(
        "Vent overlap: {}\nVent overlap with diagonals: {}",
        get_vent_overlap(FILE_PATH),
        get_vent_overlap_with_diag(FILE_PATH)
    );
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct CoordinatesPair {
    start: Coordinates,
    end: Coordinates,
}

pub fn get_vent_overlap(input_file_path: &str) -> u32 {
    let coord_v: Vec<CoordinatesPair> = format_coordinates(
        io::BufReader::new(File::open(input_file_path).expect("Could not open file")).lines(),
    );
    let mut map = HashMap::new();
    for pair in coord_v {
        let x_range = if pair.start.x <= pair.end.x {
            pair.start.x..=pair.end.x
        } else {
            pair.end.x..=pair.start.x
        };
        let y_range = if pair.start.y <= pair.end.y {
            pair.start.y..=pair.end.y
        } else {
            pair.end.y..=pair.start.y
        };

        if pair.start.x == pair.end.x {
            let x = pair.start.x;
            for y in y_range {
                let i = map.entry(Coordinates { x, y }).or_insert(0);
                *i += 1;
            }
        } else if pair.start.y == pair.end.y {
            let y = pair.start.y;
            for x in x_range {
                let i = map.entry(Coordinates { x, y }).or_insert(0);
                *i += 1;
            }
        }
    }
    let mut count = 0;
    for v in map.values() {
        if *v >= 2 {
            count += 1;
        }
    }
    count
}

pub fn get_vent_overlap_with_diag(input_file_path: &str) -> u32 {
    let coord_v: Vec<CoordinatesPair> = format_coordinates(
        io::BufReader::new(File::open(input_file_path).expect("Could not open file")).lines(),
    );
    let mut map = HashMap::new();
    for pair in coord_v {
        let x_range: Box<dyn Iterator<Item = u32>> = if pair.start.x <= pair.end.x {
            Box::new(pair.start.x..=pair.end.x)
        } else {
            Box::new((pair.end.x..=pair.start.x).rev())
        };
        let y_range: Box<dyn Iterator<Item = u32>> = if pair.start.y <= pair.end.y {
            Box::new(pair.start.y..=pair.end.y)
        } else {
            Box::new((pair.end.y..=pair.start.y).rev())
        };

        if pair.start.x == pair.end.x {
            let x = pair.start.x;
            for y in y_range {
                let i = map.entry(Coordinates { x, y }).or_insert(0);
                *i += 1;
            }
        } else if pair.start.y == pair.end.y {
            let y = pair.start.y;
            for x in x_range {
                let i = map.entry(Coordinates { x, y }).or_insert(0);
                *i += 1;
            }
        } else {
            for (x, y) in x_range.zip_eq(y_range) {
                let i = map.entry(Coordinates { x, y }).or_insert(0);
                *i += 1;
            }
        }
    }
    let mut count = 0;
    for v in map.values() {
        if *v >= 2 {
            count += 1;
        }
    }
    count
}

fn format_coordinates(lines: Lines<BufReader<File>>) -> Vec<CoordinatesPair> {
    let mut line_v: Vec<CoordinatesPair> = Vec::new();
    for line in lines {
        if let Ok(string) = line {
            let mut t = [(0, 0); 2];
            for tuple in string.split(" -> ").enumerate() {
                let st = tuple.1.split(",").collect_tuple::<(&str, &str)>().unwrap();
                t[tuple.0] = (st.0.parse().unwrap(), st.1.parse().unwrap());
            }
            line_v.push(CoordinatesPair {
                start: Coordinates {
                    x: t[0].0,
                    y: t[0].1,
                },
                end: Coordinates {
                    x: t[1].0,
                    y: t[1].1,
                },
            });
        }
    }
    line_v
}
