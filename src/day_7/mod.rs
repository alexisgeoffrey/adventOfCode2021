use itertools::Itertools;
use std::{cmp, fs::read_to_string};

pub fn crab_fuel(file_path: &str) -> i32 {
    let v = read_to_string(file_path)
        .unwrap()
        .split(',')
        .map(|crab| crab.parse::<i32>().unwrap())
        .sorted()
        .collect_vec();
    v.iter()
        .map(|crab| (cmp::max(v[v.len() / 2], *crab) - cmp::min(v[v.len() / 2], *crab)).abs())
        .sum()
}

// pub fn crab_fuel_experiment(file_path: &str) -> i32 {
//     // VERY SLOW
//     read_to_string(file_path)
//         .unwrap()
//         .split(',')
//         .map(|crab| crab.parse().unwrap())
//         .sorted()
//         .map(|crab: i32| {
//             (read_to_string(file_path)
//                 .unwrap()
//                 .split(',')
//                 .map(|crab| crab.parse::<i32>().unwrap())
//                 .sorted()
//                 .collect_vec()[read_to_string(file_path)
//                 .unwrap()
//                 .split(',')
//                 .map(|crab| crab.parse::<i32>().unwrap())
//                 .sorted()
//                 .count()
//                 / 2]
//                 - crab)
//                 .abs()
//         })
//         .sum()
// }

pub fn crab_fuel_accel(file_path: &str) -> i32 {
    let v = read_to_string(file_path)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .sorted()
        .collect_vec();

    let (avg1, avg2) = {
        let avg = f64::from(v.iter().sum::<i32>()) / v.len() as f64;
        (avg.trunc() as i32, avg.ceil() as i32)
    };

    let calc_fuel = |avg| {
        v.iter()
            .map(|crab| (0..=(cmp::max(avg, *crab) - cmp::min(avg, *crab))).sum::<i32>())
            .sum()
    };

    cmp::min(calc_fuel(avg1), calc_fuel(avg2))
}
