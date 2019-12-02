use std::fs;
use math::round;

fn get_input() -> String {
    fs::read_to_string("input/day_one.txt")
        .expect("Something went wrong reading the file")
}

pub fn one() {
    let contents = get_input();

    let total_fuel: i32 = contents
                        .lines()
                        .map(|s| s.parse::<f64>().unwrap())
                        .map(|v| (round::floor(v / 3.0, 0) - 2.0) as i32)
                        .sum();

    println!("Total fuel needed is {}", total_fuel);
}

fn calc_fuel_all(mass: f64) -> i32 {
    let mut current_mass = mass;
    let mut fuel_needed = 0;
    loop {
        current_mass = round::floor(current_mass / 3.0, 0) - 2.0;
        if current_mass >= 0.0 {
            fuel_needed += current_mass as i32;
        } else {
            break;
        }
    }
    fuel_needed
}

pub fn two() {
    let contents = get_input();

    let total_fuel: i32 = contents
                        .lines()
                        .map(|s| s.parse::<f64>().unwrap())
                        .map(|v| calc_fuel_all(v))
                        .sum();

    println!("Total fuel needed is {}", total_fuel);

}