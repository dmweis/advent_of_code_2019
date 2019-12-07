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

pub fn process_intcode(program: &mut Vec<i32>, input: Option<Vec<i32>>) -> Vec<i32> {
    let io_std = input.is_none();
    let mut input = input.unwrap_or_default();
    // Reverse vector to read data in correct order
    input.reverse();
    let mut program_counter = 0;
    let mut output_buffer = vec![];
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
                let user_input = if io_std {
                    println!("Write input pls");
                    read()
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap()
                } else {
                    input.pop().unwrap()
                };
                let output_location = program[program_counter + 1] as usize;
                program[output_location] = user_input;
                program_counter += 2;
            },
            4 => {
                // input
                let output = load_param(program, program_counter + 1, get_param_mode(op, 0));
                if io_std {
                    println!("output: {}", output);
                }
                output_buffer.push(output);
                program_counter += 2;
            },
            5 => {
                // jump if true
                let input = load_param(program, program_counter + 1, get_param_mode(op, 0));
                let target = load_param(program, program_counter + 2, get_param_mode(op, 1));
                if input != 0 {
                    program_counter = target as usize;
                } else {
                    program_counter += 3;
                }
            },
            6 => {
                // jump if false
                let input = load_param(program, program_counter + 1, get_param_mode(op, 0));
                let target = load_param(program, program_counter + 2, get_param_mode(op, 1));
                if input == 0 {
                    program_counter = target as usize;
                } else {
                    program_counter += 3;
                }
            },
            7 => {
                // less than
                let a = load_param(program, program_counter + 1, get_param_mode(op, 0));
                let b = load_param(program, program_counter + 2, get_param_mode(op, 1));
                let output = program[program_counter + 3] as usize;
                program[output] = if a < b {
                    1
                } else {
                    0
                };
                program_counter += 4;
            },
            8 => {
                // equals
                let a = load_param(program, program_counter + 1, get_param_mode(op, 0));
                let b = load_param(program, program_counter + 2, get_param_mode(op, 1));
                let output = program[program_counter + 3] as usize;
                program[output] = if a == b {
                    1
                } else {
                    0
                };
                program_counter += 4;
            },
            99 => {
                break;
            },
            _ => panic!("Error at {}", program_counter),
        }
    }
    output_buffer
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
        process_intcode(&mut input, None);
        assert_eq!(input, vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_computer_two() {
        let mut input = vec![1101,100,-1,4,0];
        process_intcode(&mut input, None);
        assert_eq!(input, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_computer_input() {
        let mut program = load_input("input/day_five.txt");
        let output = process_intcode(&mut program, Some(vec![1]));
        assert_eq!(output.last(), Some(&9006673));
    }

    #[test]
    fn jump_if_true_yes_pos_mode() {
        let mut program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = vec![6];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn jump_if_true_no_pos_mode() {
        let mut program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let input = vec![0];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn jump_if_true_yes_ime_mode() {
        let mut program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let input = vec![6];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn jump_if_true_no_ime_mode() {
        let mut program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let input = vec![0];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn is_8_pos_mode_yes() {
        let mut program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let input = vec![8];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn is_8_pos_mode_no() {
        let mut program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let input = vec![2];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn less_than_8_pos_mode_yes() {
        let mut program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let input = vec![2];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn less_than_8_pos_mode_no() {
        let mut program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let input = vec![8];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn is_8_ime_mode_yes() {
        let mut program = vec![3,3,1108,-1,8,3,4,3,99];
        let input = vec![8];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn is_8_ime_mode_no() {
        let mut program = vec![3,3,1108,-1,8,3,4,3,99];
        let input = vec![2];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn less_than_8_ime_mode_yes() {
        let mut program = vec![3,3,1107,-1,8,3,4,3,99];
        let input = vec![2];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn less_than_8_ime_mode_no() {
        let mut program = vec![3,3,1107,-1,8,3,4,3,99];
        let input = vec![8];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn long_test_day5_task_2() {
        let mut program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let input = vec![7];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![999]);

        let mut program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let input = vec![8];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1000]);

        let mut program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let input = vec![9];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![1001]);
    }

    #[test]
    fn day_5_task_2() {
        let mut program = load_input("input/day_five.txt");
        let input = vec![5];
        let output = process_intcode(&mut program, Some(input));
        assert_eq!(output, vec![3629692]);
    }
}