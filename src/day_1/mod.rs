use std::{
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

pub fn count_depth_increase(path: &str) -> u32 {
    io::BufReader::new(File::open(path).expect("Could not open file"))
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .tuple_windows()
        .fold(0, |accum, (prev, next)| {
            if next > prev {
                return accum + 1;
            }
            accum
        })
}

pub fn count_sliding_depth_increase(path: &str) -> u32 {
    io::BufReader::new(File::open(path).expect("Could not open file"))
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |accum, (prev, next)| {
            if next > prev {
                return accum + 1;
            }
            accum
        })
}
