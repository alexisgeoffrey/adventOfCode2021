use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

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
    let mut tabulate = |x_stable, stable_coord, range| {
        for coord in range {
            let key = if x_stable {
                Coordinates {
                    x: stable_coord,
                    y: coord,
                }
            } else {
                Coordinates {
                    x: coord,
                    y: stable_coord,
                }
            };
            *map.entry(key).or_insert(0) += 1;
        }
    };

    for pair in coord_v {
        if pair.start.x == pair.end.x {
            tabulate(true, pair.start.x, get_range(pair.start.y, pair.end.y));
        } else if pair.start.y == pair.end.y {
            tabulate(false, pair.start.y, get_range(pair.start.x, pair.end.x));
        }
    }

    map.values()
        .fold(0, |acc, vent| if *vent >= 2 { acc + 1 } else { acc })
}

pub fn get_vent_overlap_with_diag(input_file_path: &str) -> usize {
    let coord_v: Vec<CoordinatesPair> = format_coordinates(
        io::BufReader::new(File::open(input_file_path).expect("Could not open file")).lines(),
    );
    let mut map = HashMap::new();
    let tabulate = |x_stable, stable_coord, range, map: &mut HashMap<Coordinates, u32>| {
        for coord in range {
            let key = if x_stable {
                Coordinates {
                    x: stable_coord,
                    y: coord,
                }
            } else {
                Coordinates {
                    x: coord,
                    y: stable_coord,
                }
            };
            *map.entry(key).or_insert(0) += 1;
        }
    };

    for pair in coord_v {
        if pair.start.x == pair.end.x {
            tabulate(
                true,
                pair.start.x,
                get_range(pair.start.y, pair.end.y),
                &mut map,
            );
        } else if pair.start.y == pair.end.y {
            tabulate(
                false,
                pair.start.y,
                get_range(pair.start.x, pair.end.x),
                &mut map,
            );
        } else {
            for (x, y) in
                get_range(pair.start.x, pair.end.x).zip_eq(get_range(pair.start.y, pair.end.y))
            {
                *map.entry(Coordinates { x, y }).or_insert(0) += 1;
            }
        }
    }

    map.values().filter(|x| **x >= 2).count()
}

fn get_range(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
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

fn format_coordinates(lines: io::Lines<io::BufReader<File>>) -> Vec<CoordinatesPair> {
    lines
        .map(|s| {
            let i = s
                .unwrap()
                .split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();
            CoordinatesPair {
                start: Coordinates { x: i.0, y: i.1 },
                end: Coordinates { x: i.2, y: i.3 },
            }
        })
        .collect_vec()
}
