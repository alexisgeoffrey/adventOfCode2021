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

#[derive(Eq, PartialEq, Hash)]
struct Coordinates {
    x: u32,
    y: u32,
}

struct CoordinatesPair {
    start: Coordinates,
    end: Coordinates,
}

pub fn get_vent_overlap(input_file_path: &str) -> usize {
    let coord_v: Vec<CoordinatesPair> = format_coordinates(
        io::BufReader::new(File::open(input_file_path).expect("Could not open file")).lines(),
    );
    let mut map = HashMap::new();
    for pair in coord_v {
        let tabulate = |x_stable: bool,
                        stable_coord: u32,
                        range: Box<dyn Iterator<Item = u32>>,
                        map: &mut HashMap<Coordinates, i32>| {
            let x = stable_coord;
            for y in range {
                let key = if x_stable {
                    Coordinates { x, y }
                } else {
                    Coordinates { x: y, y: x }
                };
                let i = map.entry(key).or_insert(0);
                *i += 1;
            }
        };

        if pair.start.x == pair.end.x {
            tabulate(
                true,
                pair.start.x,
                range(pair.start.y, pair.end.y),
                &mut map,
            );
        } else if pair.start.y == pair.end.y {
            tabulate(
                false,
                pair.start.y,
                range(pair.start.x, pair.end.x),
                &mut map,
            );
        }
    }

    map.values().filter(|x| **x >= 2).count()
}

pub fn get_vent_overlap_with_diag(input_file_path: &str) -> usize {
    let coord_v: Vec<CoordinatesPair> = format_coordinates(
        io::BufReader::new(File::open(input_file_path).expect("Could not open file")).lines(),
    );
    let mut map = HashMap::new();
    for pair in coord_v {
        let tabulate = |x_stable: bool,
                        stable_coord: u32,
                        range: Box<dyn Iterator<Item = u32>>,
                        map: &mut HashMap<Coordinates, i32>| {
            let x = stable_coord;
            for y in range {
                let key = if x_stable {
                    Coordinates { x, y }
                } else {
                    Coordinates { x: y, y: x }
                };
                let i = map.entry(key).or_insert(0);
                *i += 1;
            }
        };

        if pair.start.x == pair.end.x {
            tabulate(
                true,
                pair.start.x,
                range(pair.start.y, pair.end.y),
                &mut map,
            );
        } else if pair.start.y == pair.end.y {
            tabulate(
                false,
                pair.start.y,
                range(pair.start.x, pair.end.x),
                &mut map,
            );
        } else {
            for (x, y) in range(pair.start.x, pair.end.x).zip_eq(range(pair.start.y, pair.end.y)) {
                let i = map.entry(Coordinates { x, y }).or_insert(0);
                *i += 1;
            }
        }
    }

    map.values().filter(|x| **x >= 2).count()
}

fn range(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start <= end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

// fn format_coordinates_v1(lines: Lines<BufReader<File>>) -> Vec<CoordinatesPair> {
//     let mut line_v: Vec<CoordinatesPair> = Vec::new();
//     for line in lines.flatten() {
//         let mut t = [(0, 0); 2];
//         for tuple in line.split(" -> ").enumerate() {
//             let st = tuple.1.split(',').collect_tuple::<(&str, &str)>().unwrap();
//             t[tuple.0] = (st.0.parse().unwrap(), st.1.parse().unwrap());
//         }
//         line_v.push(CoordinatesPair {
//             start: Coordinates {
//                 x: t[0].0,
//                 y: t[0].1,
//             },
//             end: Coordinates {
//                 x: t[1].0,
//                 y: t[1].1,
//             },
//         });
//     }
//     line_v
// }

fn format_coordinates(lines: Lines<BufReader<File>>) -> Vec<CoordinatesPair> {
    lines
        .flatten()
        .map(|x| {
            x.split(" -> ")
                .map(|x| x.split(','))
                .flatten()
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32, u32)>()
                .unwrap()
        })
        .map(|i| CoordinatesPair {
            start: Coordinates { x: i.0, y: i.1 },
            end: Coordinates { x: i.2, y: i.3 },
        })
        .collect_vec()
}
