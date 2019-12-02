use std::fs;

fn get_input() -> Vec<i32> {
    fs::read_to_string("input/day_two.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

pub fn process_intcode(program: &mut Vec<i32>) {
    let mut program_counter = 0;
    loop {
        let op = program[program_counter];
        match op {
            1 => {
                // addition
                let a = program[program[program_counter + 1] as usize];
                let b = program[program[program_counter + 2] as usize];
                let output_location = program[program_counter + 3] as usize;
                program[output_location] = a + b;
                program_counter += 4;

            },
            2 => {
                // multiplication
                let a = program[program[program_counter + 1] as usize];
                let b = program[program[program_counter + 2] as usize];
                let output_location = program[program_counter + 3] as usize;
                program[output_location] = a * b;
                program_counter += 4;
            },
            99 => {
                break;
            },
            _ => panic!("Error at {}", program_counter),
        }
    }
}

pub fn one() {
    let mut program = get_input();

    // set initial state
    program[1] = 12;
    program[2] = 2;

    process_intcode(&mut program);

    println!("Done. Position 0 has value {}", program[0]);
}

pub fn two() {
    let program = get_input();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut current_program = program.clone();
            current_program[1] = noun;
            current_program[2] = verb;
            process_intcode(&mut current_program);
            if current_program[0] == 19690720 {
                println!("Works for noun {} verb {} => code is {}", noun, verb, 100 * noun + verb);
            }
        }
    }
}
