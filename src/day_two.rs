use std::fs;

fn get_input() -> Vec<i32> {
    fs::read_to_string("input/day_two.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn one() {
    let mut program = get_input();

    // set initial state
    program[1] = 12;
    program[2] = 2;

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

    println!("Done. Position 0 has value {}", program[0]);
}
