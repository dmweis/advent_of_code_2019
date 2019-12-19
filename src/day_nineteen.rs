use crate::infinite_memory_intcomputer::{IntcodeComputer, IntcodeComputerState::OutputProduced};
use std::collections::{HashMap, HashSet};

const STATIONARY: i64 = 0;
const MOVING: i64 = 1;

pub fn one() {
    let mut map = HashMap::new();
    for y in 0..50 {
        for x in 0..50 {
            let mut computer = IntcodeComputer::new_from_file("input/day_nineteen.txt");
            computer.provide_input(x);
            computer.provide_input(y);
            if let OutputProduced(output) = computer.run().unwrap() {
                map.insert((x, y), output);
                if output == STATIONARY {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                panic!("Computer didn't return value {} {}", x, y);
            }
        }
        println!();
    }
    let mut affected = 0;
    for (_, effect) in map {
        if effect == MOVING {
            affected += 1;
        }
    }
    println!("{}", affected);
}

fn check_pos(pos: &(i64, i64), computer: &IntcodeComputer) -> bool {
    let mut local_computer = computer.clone();
    local_computer.provide_input(pos.0);
    local_computer.provide_input(pos.1);
    if let OutputProduced(res) = local_computer.run().unwrap() {
         res == MOVING
    } else {
        false
    }
}

fn check_ship(far_left: &(i64, i64), map: &HashSet<(i64, i64)>) -> bool {
    for y in 0..100 {
        for x in 0..100 {
            if !map.contains(&(far_left.0+x, far_left.1-y)){
                return false;
            }
        }
    }
    true
}

pub fn two() {
    let computer = IntcodeComputer::new_from_file("input/day_nineteen.txt");
    let mut map = HashSet::new();
    let mut y = 20;
    let mut start_x = 0;
    loop {
        // scan line
        let mut current_line = vec![];
        let mut current_x = start_x;
        // look for elements in line
        loop {
            if current_line.is_empty() {
                if check_pos(&(current_x, y), &computer) {
                    current_line.push(current_x)
                }
            } else {
                if !check_pos(&(current_x, y), &computer) {
                    break;
                }
                current_line.push(current_x)
            }
            current_x+=1;
        }
        // line is in current_line now
        // check ship
        start_x = current_line[0] - 2;
        for x in &current_line {
            map.insert((*x, y));
        }
        if check_ship(&(current_line[0], y), &map) {
            let closest_x = current_line[0];
            let farthest_x = current_line[0] + 99;
            let closest_y = y - 99;
            let farthest_y = y;
            // debug draw
            for y in (closest_y - 5)..(closest_y + 110) {
                for x in (closest_x - 5)..(closest_x + 110) {
                    if map.contains(&(x, y)) {
                        if x >= closest_x && x <= farthest_x && y >= closest_y && y <= farthest_y {
                            print!("O");
                        } else {
                            print!("#");
                        }
                    } else {
                        print!(".");
                    }
                }
                println!();
            }

            println!("closest point coordinates {}", closest_x * 10000 + closest_y);
            return;
        }
        println!("current width {} at line {}", current_line.len(), y);
        y+=1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}