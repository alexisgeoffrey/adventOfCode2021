use std::fs;

const FILE_PATH: &str = "C:/Users/Alexis/dev/adventofcode/2021/day_6/src/input.txt";

fn main() {
    println!(
        "Lanternfish after 80 days: {}",
        lanternfish_sim(FILE_PATH, 80)
    );
    println!(
        "Lanternfish after 256 days: {}",
        lanternfish_sim(FILE_PATH, 256)
    );
}

pub fn lanternfish_sim(file_path: &str, days: u32) -> u128 {
    let mut colony: [u128; 9] = [0; 9];
    let f = fs::read_to_string(file_path).expect("Error reading file");
    for element in f.split(",") {
        let num = element.parse::<usize>().unwrap();
        colony[num] += 1;
    }

    for _day in 0..days {
        let mut new_fish = 0;
        if colony[0] > 0 {
            new_fish = colony[0];
            colony[0] = 0;
        }
        for nums in 1..colony.len() {
            colony[nums - 1] = colony[nums];
            colony[nums] = 0;
        }
        colony[8] = new_fish;
        colony[6] += new_fish;
    }

    let mut sum = 0;
    for i in 0..colony.len() {
        sum += colony[i];
    }
    sum
}
