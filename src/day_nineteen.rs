use crate::infinite_memory_intcomputer::{IntcodeComputer, IntcodeComputerState::OutputProduced};
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

}