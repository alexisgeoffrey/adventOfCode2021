use itertools::Itertools;
use std::{cmp, fs::read_to_string};

const FILE_PATH: &str = "C:/Users/Alexis/dev/adventofcode/2021/day_7/src/input.txt";

fn main() {
    println!(
        "Fuel required: {}\nFuel required corrected: {}",
        crab_fuel(FILE_PATH),
        crab_fuel_accel(FILE_PATH)
    );
    println!("Experiment: {}", crab_fuel_experiment(FILE_PATH))
}

pub fn crab_fuel(file_path: &str) -> i32 {
    let v = read_to_string(file_path)
        .unwrap()
        .split(",")
        .map(|crab| crab.parse().unwrap())
        .sorted()
        .collect::<Vec<i32>>();
    v.iter()
        .map(|crab| (cmp::max(v[v.len() / 2], *crab) - cmp::min(v[v.len() / 2], *crab)).abs())
        .sum()
}

pub fn crab_fuel_experiment(file_path: &str) -> i32 {
    // VERY SLOW
    read_to_string(file_path)
        .unwrap()
        .split(",")
        .map(|crab| crab.parse().unwrap())
        .sorted()
        .map(|crab: i32| {
            (read_to_string(file_path)
                .unwrap()
                .split(",")
                .map(|crab| crab.parse().unwrap())
                .sorted()
                .collect::<Vec<i32>>()[read_to_string(file_path)
                .unwrap()
                .split(",")
                .map(|crab| crab.parse().unwrap())
                .sorted()
                .collect::<Vec<i32>>()
                .len()
                / 2]
                - crab)
                .abs()
        })
        .sum()
}

pub fn crab_fuel_accel(file_path: &str) -> i32 {
    let v = read_to_string(file_path)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .sorted()
        .collect::<Vec<i32>>();

    let (avg1, avg2) = {
        let avg = v.iter().sum::<i32>() as f32 / v.len() as f32;
        (avg.trunc(), avg.ceil())
    };

    let calc_fuel = |avg| {
        v.iter()
            .map(|crab| {
                (0..=(cmp::max(avg as i32, *crab) - cmp::min(avg as i32, *crab))).sum::<i32>()
            })
            .sum()
    };
    cmp::min(calc_fuel(avg1), calc_fuel(avg2))
}
