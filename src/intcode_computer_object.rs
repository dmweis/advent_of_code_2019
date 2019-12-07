use std::fs;
use std::io;
use std::error::*;

#[derive(Debug)]
pub enum IntcodeComputerState {
    WaitingForInput,
    Halted,
    OutputProduced(i64),
}

pub struct IntcodeComputer {
    pub program: Vec<i64>,
    instruction_pointer: usize,
    last_input_pointer: Option<usize>,
}

impl IntcodeComputer {
    pub fn new(program: Vec<i64>) -> IntcodeComputer {
        IntcodeComputer {
            program,
            instruction_pointer: 0,
            last_input_pointer: None,
        }
    }

    pub fn new_from_file(path: &str) -> IntcodeComputer {
        IntcodeComputer::new(load_input(path))
    }

    pub fn provide_input(&mut self, input: i64) -> Result<(), Box<dyn Error>> {
        let input_pointer = self.last_input_pointer.ok_or("No input expected")?;
        self.program[input_pointer] = input;
        self.last_input_pointer = None;
        Ok(())
    }

    pub fn run(&mut self) -> IntcodeComputerState {
        if let Some(_) = self.last_input_pointer {
            return IntcodeComputerState::WaitingForInput;
        }
        loop {
            if self.instruction_pointer >= self.program.len() {
                return IntcodeComputerState::Halted;
            }
            let op = self.program[self.instruction_pointer];
            match get_op_code(op) {
                1 => {
                    // addition
                    let a = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    let b = load_param(&self.program, self.instruction_pointer + 2, get_param_mode(op, 1));
                    let output_location = self.program[self.instruction_pointer + 3] as usize;
                    self.program[output_location] = a + b;
                    self.instruction_pointer += 4;
                },
                2 => {
                    // multiplication
                    let a = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    let b = load_param(&self.program, self.instruction_pointer + 2, get_param_mode(op, 1));
                    let output_location = self.program[self.instruction_pointer + 3] as usize;
                    self.program[output_location] = a * b;
                    self.instruction_pointer += 4;
                },
                3 => {
                    // input
                    let output_location = self.program[self.instruction_pointer + 1] as usize;
                    self.last_input_pointer = Some(output_location);
                    self.instruction_pointer += 2;
                    return IntcodeComputerState::WaitingForInput;
                },
                4 => {
                    // output
                    let output = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    self.instruction_pointer += 2;
                    return IntcodeComputerState::OutputProduced(output);
                },
                5 => {
                    // jump if true
                    let input = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    let target = load_param(&self.program, self.instruction_pointer + 2, get_param_mode(op, 1));
                    if input != 0 {
                        self.instruction_pointer = target as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                6 => {
                    // jump if false
                    let input = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    let target = load_param(&self.program, self.instruction_pointer + 2, get_param_mode(op, 1));
                    if input == 0 {
                        self.instruction_pointer = target as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                7 => {
                    // less than
                    let a = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    let b = load_param(&self.program, self.instruction_pointer + 2, get_param_mode(op, 1));
                    let output = self.program[self.instruction_pointer + 3] as usize;
                    self.program[output] = if a < b { 1 } else { 0 };
                    self.instruction_pointer += 4;
                },
                8 => {
                    // equals
                    let a = load_param(&self.program, self.instruction_pointer + 1, get_param_mode(op, 0));
                    let b = load_param(&self.program, self.instruction_pointer + 2, get_param_mode(op, 1));
                    let output = self.program[self.instruction_pointer + 3] as usize;
                    self.program[output] = if a == b { 1 } else { 0 };
                    self.instruction_pointer += 4;
                },
                99 => {
                    return IntcodeComputerState::Halted;
                },
                _ => panic!("Error at {}", self.instruction_pointer),
            }
        }
    }
}

fn read() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn load_input(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

#[derive(PartialEq, Debug)]
enum ParamMode {
    PositionMode,
    ImmediateMode,
}

fn get_op_code(op: i64) -> i64 {
    op % 100
}

fn get_param_mode(op: i64, param_index: u32) -> ParamMode {
    let mode = op / (10_i64.pow(param_index + 2)) % 10;
    match mode {
        0 => ParamMode::PositionMode,
        1 => ParamMode::ImmediateMode,
        _ => panic!("Unknown param mode"),
    }
}

fn load_param(data: &Vec<i64>, index: usize, mode: ParamMode) -> i64 {
    match mode {
        ParamMode::ImmediateMode => data[index],
        ParamMode::PositionMode => data[data[index] as usize],
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_op_code_extracts() {
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
        let input = vec![1002, 4, 3, 4, 33];
        let mut computer = IntcodeComputer::new(input);
        computer.run();
        assert_eq!(computer.program, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_computer_two() {
        let input = vec![1101, 100, -1, 4, 0];
        let mut computer = IntcodeComputer::new(input);
        computer.run();
        assert_eq!(computer.program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_computer_input() {
        let program = load_input("input/day_five.txt");
        let mut computer = IntcodeComputer::new(program);
        let mut output = -1;
        loop {
            match computer.run() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(1).unwrap() 
            }
        }
        assert_eq!(output, 9006673);
    }

    #[test]
    fn jump_if_true_yes_pos_mode() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut computer = IntcodeComputer::new(program);
        let mut output = -1;
        loop {
            match computer.run() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(6).unwrap() 
            }
        }
        assert_eq!(output, 1);
    }

    #[test]
    fn jump_if_true_no_pos_mode() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut computer = IntcodeComputer::new(program);
        let mut output = -1;
        loop {
            match computer.run() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(0).unwrap() 
            }
        }
        assert_eq!(output, 0);
    }

    #[test]
    fn jump_if_true_yes_ime_mode() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut computer = IntcodeComputer::new(program);
        let mut output = -1;
        loop {
            match computer.run() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(6).unwrap() 
            }
        }
        assert_eq!(output, 1);
    }

    #[test]
    fn jump_if_true_no_ime_mode() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut computer = IntcodeComputer::new(program);
        let mut output = -1;
        loop {
            match computer.run() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(0).unwrap() 
            }
        }
        assert_eq!(output, 0);
    }

    fn test_wrapper(program: &Vec<i64>, input: Option<Vec<i64>>) -> Vec<i64> {
        let mut input = input.unwrap();
        input.reverse();
        let mut computer = IntcodeComputer::new(program.clone());
        let mut output = -1;
        loop {
            match computer.run() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(input.pop().unwrap()).unwrap() 
            }
        }
        vec![output]
    }

    #[test]
    fn is_8_pos_mode_yes() {
        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = vec![8];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn is_8_pos_mode_no() {
        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = vec![2];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn less_than_8_pos_mode_yes() {
        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = vec![2];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn less_than_8_pos_mode_no() {
        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = vec![8];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn is_8_ime_mode_yes() {
        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let input = vec![8];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn is_8_ime_mode_no() {
        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let input = vec![2];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn less_than_8_ime_mode_yes() {
        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input = vec![2];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![1]);
    }

    #[test]
    fn less_than_8_ime_mode_no() {
        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input = vec![8];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![0]);
    }

    #[test]
    fn long_test_day5_task_2() {
        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input = vec![7];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![999]);

        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input = vec![8];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![1000]);

        let mut program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input = vec![9];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![1001]);
    }

    #[test]
    fn day_5_task_2() {
        let mut program = load_input("input/day_five.txt");
        let input = vec![5];
        let output = test_wrapper(&mut program, Some(input));
        assert_eq!(output, vec![3629692]);
    }
}
