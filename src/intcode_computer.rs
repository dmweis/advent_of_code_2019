use std::fs;
use std::io;

fn read() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn load_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .split(",")
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

#[derive(PartialEq, Debug)]
enum ParamMode {
    PositionMode,
    ImmediateMode
}

fn get_op_code(op: i32) -> i32 {
    op % 100
}

fn get_param_mode(op: i32, param_index: u32) -> ParamMode{
    let mode = op / (10_i32.pow(param_index+2)) % 10;
    match mode {
        0 => ParamMode::PositionMode,
        1 => ParamMode::ImmediateMode,
        _ => panic!("Unknown param mode"),
    }
}

fn load_param(data: &Vec<i32>, index: usize, mode: ParamMode) -> i32 {
    match mode {
        ParamMode::ImmediateMode => data[index],
        ParamMode::PositionMode => data[data[index] as usize],
    }
}

pub fn process_intcode(program: &mut Vec<i32>) {
    let mut program_counter = 0;
    loop {
        let op = program[program_counter];
        match get_op_code(op) {
            1 => {
                // addition
                let a = load_param(program, program_counter + 1, get_param_mode(op, 0));
                let b = load_param(program, program_counter + 2, get_param_mode(op, 1));
                let output_location = program[program_counter + 3] as usize;
                program[output_location] = a + b;
                program_counter += 4;
            },
            2 => {
                // multiplication
                let a = load_param(program, program_counter + 1, get_param_mode(op, 0));
                let b = load_param(program, program_counter + 2, get_param_mode(op, 1));
                let output_location = program[program_counter + 3] as usize;
                program[output_location] = a * b;
                program_counter += 4;
            },
            3 => {
                // input
                println!("Write input pls");
                let input = read().unwrap();
                let output_location = program[program_counter + 1] as usize;
                program[output_location] = input.trim().parse().unwrap();
                program_counter += 2;
            },
            4 => {
                // input
                let output = load_param(program, program_counter + 1, get_param_mode(op, 0));
                println!("output: {}", output);
                program_counter += 2;
            },
            99 => {
                break;
            },
            _ => panic!("Error at {}", program_counter),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_op_code_extracts(){
        assert_eq!(get_op_code(1002), 02);
        assert_eq!(get_op_code(2), 02);
    }

    #[test]
    fn get_param_mode_one() {
        assert_eq!(get_param_mode(1002, 0), ParamMode::PositionMode);
        assert_eq!(get_param_mode(1002, 1), ParamMode::ImmediateMode);
        assert_eq!(get_param_mode(1002, 2), ParamMode::PositionMode);
    }

    #[test]
    fn test_load_param_immediate() {
        let data = vec![7];
        assert_eq!(load_param(&data, 0, ParamMode::ImmediateMode), 7);
    }

    #[test]
    fn test_load_param_position() {
        let data = vec![2, 4, 5, 6];
        assert_eq!(load_param(&data, 0, ParamMode::PositionMode), 5);
    }

    #[test]
    fn test_computer_one() {
        let mut input = vec![1002,4,3,4,33];
        process_intcode(&mut input);
        assert_eq!(input, vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_computer_two() {
        let mut input = vec![1101,100,-1,4,0];
        process_intcode(&mut input);
        assert_eq!(input, vec![1101,100,-1,4,99]);
    }
}