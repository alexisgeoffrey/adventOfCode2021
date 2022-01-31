use std::{
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

pub fn path_reader(path: &str) -> u32 {
    let t = io::BufReader::new(File::open(path).expect("Could not open file"))
        .lines()
        .map(Result::unwrap)
        .fold((0, 0), |acc, line| {
            let line_tokens = line.split_whitespace().collect_tuple::<(_, _)>().unwrap();
            match line_tokens.0 {
                "forward" => (acc.0 + line_tokens.1.parse::<u32>().unwrap(), acc.1),
                "down" => (acc.0, acc.1 + line_tokens.1.parse::<u32>().unwrap()),
                "up" => (acc.0, acc.1 - line_tokens.1.parse::<u32>().unwrap()),
                _ => acc,
            }
        });

    t.0 * t.1
}

pub fn aimed_path_reader(path: &str) -> u32 {
    let t = io::BufReader::new(File::open(path).expect("Could not open file"))
        .lines()
        .map(Result::unwrap)
        .fold((0, 0, 0), |acc, line| {
            let line_tokens = line.split_whitespace().collect_tuple::<(_, _)>().unwrap();
            match line_tokens.0 {
                "forward" => (
                    acc.0 + line_tokens.1.parse::<u32>().unwrap(),
                    acc.1 + acc.2 * line_tokens.1.parse::<u32>().unwrap(),
                    acc.2,
                ),
                "down" => (acc.0, acc.1, acc.2 + line_tokens.1.parse::<u32>().unwrap()),
                "up" => (acc.0, acc.1, acc.2 - line_tokens.1.parse::<u32>().unwrap()),
                _ => acc,
            }
        });

    t.0 * t.1
}
