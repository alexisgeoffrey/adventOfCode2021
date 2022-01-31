mod day_5;
mod day_6;
mod day_7;

fn main() {
    let day_5_path = "src/day_5/day_5.txt";
    let day_6_path = "src/day_6/day_6.txt";
    let day_7_path = "src/day_7/day_7.txt";

    println!("Day 5:");
    println!(
        "Vent overlap: {}\nVent overlap with diagonals: {}",
        day_5::get_vent_overlap(day_5_path),
        day_5::get_vent_overlap_with_diag(day_5_path)
    );
    println!();

    println!("Day 6:");
    println!(
        "Lanternfish after 80 days: {}",
        day_6::lanternfish_sim(day_6_path, 80)
    );
    println!(
        "Lanternfish after 256 days: {}",
        day_6::lanternfish_sim(day_6_path, 256)
    );
    println!(
        "Lanternfish after 1000 days: {}",
        day_6::lanternfish_sim(day_6_path, 1000)
    );
    println!(
        "Lanternfish after 5000 days: {}",
        day_6::lanternfish_sim(day_6_path, 5000)
    );
    println!(
        "Lanternfish after 10000 days: {}",
        day_6::lanternfish_sim(day_6_path, 10000)
    );
    println!();

    println!("Day 7:");
    println!(
        "Fuel required: {}\nFuel required corrected: {}",
        day_7::crab_fuel(day_7_path),
        day_7::crab_fuel_accel(day_7_path)
    );
}
