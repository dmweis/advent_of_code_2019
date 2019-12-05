use crate::intcode_computer::*;


pub fn one() {
    let mut program = load_input("input/day_five.txt");
    process_intcode(&mut program)
}

// pub fn two() {
//     let program = get_input();

//     for noun in 0..100 {
//         for verb in 0..100 {
//             let mut current_program = program.clone();
//             current_program[1] = noun;
//             current_program[2] = verb;
//             process_intcode(&mut current_program);
//             if current_program[0] == 19690720 {
//                 println!("Works for noun {} verb {} => code is {}", noun, verb, 100 * noun + verb);
//             }
//         }
//     }
// }
