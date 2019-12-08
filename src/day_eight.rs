use std::fs;
use itertools::Itertools;

fn get_input() -> String {
    fs::read_to_string("input/day_eight.txt")
        .expect("Something went wrong reading the file")
}

const WIDTH: i32 = 25;
const HEIGHT: i32 = 6;

pub fn one() {
    let input_text = get_input();
    let layers = input_text
        .chars()
        .filter_map(|letter| letter.to_digit(10))
        .chunks((WIDTH * HEIGHT) as usize);
    let mut counter = vec![];
    for layer in &layers {
        let layer = layer.collect::<Vec<u32>>();
        let zero_count = layer.iter().filter(|digit| **digit == 0).count();
        let one_count = layer.iter().filter(|digit| **digit == 1).count();
        let two_count = layer.iter().filter(|digit| **digit == 2).count();

        counter.push((zero_count, one_count * two_count));
    }

    let min = counter.iter().min_by_key(|count| count.0);
    println!("{:?}", min);
}