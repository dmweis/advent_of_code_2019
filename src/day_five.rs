use crate::intcode_computer::*;


pub fn one() {
    let mut program = load_input("input/day_five.txt");
    process_intcode(&mut program, None);
}

pub fn two() {
    let mut program = load_input("input/day_five.txt");
    process_intcode(&mut program, None);
}
