use std::fs;
use std::collections::HashMap;

fn get_input() -> String {
    fs::read_to_string("input/day_twenty.txt")
        .expect("Something went wrong reading the file")
}

pub fn one() {
    let input = get_input();
    let mut map = HashMap::new();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter != ' ' {
                map.insert((x, y), letter);
            }
        }
    }

    let mut start = (0, 0);
    let mut end = (0, 0);
    // let mut tunnels = HashMap::new();
    //find keys
    for y in 0..height {
        for x in 0..width {
            // vertical
            
        }
    }
}