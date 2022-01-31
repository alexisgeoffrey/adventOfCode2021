use num_bigint::BigUint;
use num_traits::{one, zero};
use std::fs;

pub fn lanternfish_sim(file_path: &str, days: u32) -> BigUint {
    let mut colony: [BigUint; 9] = Default::default();
    let input = fs::read_to_string(file_path).expect("Error reading file");
    for element in input.split(',') {
        let num = element.parse::<usize>().unwrap();
        colony[num] += one::<BigUint>();
    }

    for _day in 0..days {
        let mut new_fish = zero();
        if colony[0] > one() {
            new_fish = colony[0].clone();
            colony[0] = zero();
        }
        for num in 1..colony.len() {
            colony[num - 1] = colony[num].clone();
            colony[num] = zero();
        }
        colony[8] = new_fish.clone();
        colony[6] += new_fish;
    }

    colony.iter().sum()
}
