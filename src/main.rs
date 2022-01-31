mod day_1;
mod day_2;
// mod day_3;
// mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
    println!("Day 1:");
    println!(
        "Depth increase: {}\nSliding depth increase: {}",
        day_1::count_depth_increase(&path("1")),
        day_1::count_sliding_depth_increase(&path("1"))
    );
    println!();

    println!("Day 2:");
    println!(
        "Multiplied position and depth: {}\nMultiplied aimed position and depth: {}",
        day_2::path_reader(&path("2")),
        day_2::aimed_path_reader(&path("2"))
    );
    println!();

    println!("Day 5:");
    println!(
        "Vent overlap: {}\nVent overlap with diagonals: {}",
        day_5::get_vent_overlap(&path("5")),
        day_5::get_vent_overlap_with_diag(&path("5"))
    );
    println!();

    println!("Day 6:");
    println!(
        "Lanternfish after 80 days: {}",
        day_6::lanternfish_sim(&path("6"), 80)
    );
    println!(
        "Lanternfish after 256 days: {}",
        day_6::lanternfish_sim(&path("6"), 256)
    );
    println!(
        "Lanternfish after 1000 days: {}",
        day_6::lanternfish_sim(&path("6"), 1000)
    );
    println!(
        "Lanternfish after 5000 days: {}",
        day_6::lanternfish_sim(&path("6"), 5000)
    );
    println!(
        "Lanternfish after 10000 days: {}",
        day_6::lanternfish_sim(&path("6"), 10000)
    );
    println!();

    println!("Day 7:");
    println!(
        "Fuel required: {}\nFuel required corrected: {}",
        day_7::crab_fuel(&path("7")),
        day_7::crab_fuel_accel(&path("7"))
    );
}

fn path(day: &str) -> String {
    format!("src/day_{day}/day_{day}.txt")
}
