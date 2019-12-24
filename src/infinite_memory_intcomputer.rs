use std::fs;
use std::error::*;
use std::collections::{VecDeque, HashMap};

#[derive(Debug)]
pub enum IntcodeComputerState {
    WaitingForInput,
    Halted,
    OutputProduced(i64),
}

#[derive(PartialEq, Debug)]
pub enum ParamMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
}

#[derive(Clone)]
pub struct IntcodeComputer {
    memory: HashMap<i32, i64>,
    instruction_pointer: i32,
    relative_base: i32,
    input_queue: VecDeque<i64>,
    output: Vec<i64>,
    output_queue: VecDeque<i64>,
}

impl IntcodeComputer {
    pub fn new(program: Vec<i64>) -> IntcodeComputer {
        let memory: HashMap<i32, i64> = program
                                            .iter()
                                            .enumerate()
                                            .map(|(i, v)| (i as i32, *v))
                                            .collect();
        IntcodeComputer {
            memory,
            instruction_pointer: 0,
            relative_base: 0,
            input_queue: VecDeque::new(),
            output: vec![],
            output_queue: VecDeque::new(),
        }
    }

    pub fn new_from_file(path: &str) -> IntcodeComputer {
        IntcodeComputer::new(load_input(path))
    }

    pub fn provide_input(&mut self, input: i64) {
        self.input_queue.push_back(input);
    }

    pub fn provide_input_iter<T: IntoIterator<Item=i64>>(&mut self, input: T) {
        for value in input {
            self.input_queue.push_back(value);
        }
    }

    fn read_memory(&self, location: &i32) -> Result<i64, Box<dyn Error>> {
        if location < &0 {
            Err("Accessing memory in negative index")?;
        }
        Ok(*self.memory.get(&location).unwrap_or(&0))
    }

    pub fn write_memory(&mut self, location: i32, value: i64, mode: ParamMode) -> Result<(), Box<dyn Error>> {
        let location = match mode {
            ParamMode::ImmediateMode => panic!("wrong param mode for write"),
            ParamMode::PositionMode => location,
            ParamMode::RelativeMode => location + self.relative_base,
        };
        if location < 0 {
            Err("Writing memory in negative index")?;
        }
        self.memory.insert(location, value);
        Ok(())
    }

    fn load_param(&self, location: &i32, mode: ParamMode) -> Result<i64, Box<dyn Error>> {
        match mode {
            ParamMode::ImmediateMode => Ok(self.read_memory(location)?),
            ParamMode::PositionMode => {
                let absolute_position = self.read_memory(location)?;
                Ok(self.read_memory(&(absolute_position as i32))?)
            },
            ParamMode::RelativeMode => {
                let relative_position = self.read_memory(location)? as i32;
                Ok(self.read_memory(&(relative_position + self.relative_base))?)
            }
        }
    }

    pub fn dump_memory(&self) -> Vec<i64> {
        let max = self.memory.keys().max().unwrap_or(&0) + 1;
        let mut memory_vec = Vec::with_capacity(max as usize);
        for key in 0..max {
            memory_vec.push(self.read_memory(&key).unwrap());
        }
        memory_vec
    }

    pub fn get_output(&self) -> Vec<i64> {
        self.output.clone()
    }

    pub fn pop_output(&mut self) -> Vec<i64> {
        let mut output = vec![];
        while let Some(val) = self.output_queue.pop_front() {
            output.push(val);
        }
        output
    }

    pub fn run_ignore_output(&mut self) -> Result<IntcodeComputerState, Box<dyn Error>> {
        loop {
            match self.run()? {
                IntcodeComputerState::Halted => return Ok(IntcodeComputerState::Halted),
                IntcodeComputerState::WaitingForInput => return Ok(IntcodeComputerState::WaitingForInput),
                IntcodeComputerState::OutputProduced(_) => (),
            }
        }
    }

    pub fn run(&mut self) -> Result<IntcodeComputerState, Box<dyn Error>> {
        loop {
            // let op = self.memory.get(&self.instruction_pointer)
            let op = &self.read_memory(&self.instruction_pointer)?;
            match get_op_code(op) {
                1 => {
                    // addition
                    let a = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    let b = self.load_param(&(self.instruction_pointer + 2), get_param_mode(op, 1))?;
                    let output_location = self.load_param(&(self.instruction_pointer + 3), ParamMode::ImmediateMode)?;
                    self.write_memory(output_location as i32, a + b, get_param_mode(op, 2))?;
                    self.instruction_pointer += 4;
                },
                2 => {
                    // multiplication
                    let a = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    let b = self.load_param(&(self.instruction_pointer + 2), get_param_mode(op, 1))?;
                    let output_location = self.load_param(&(self.instruction_pointer + 3), ParamMode::ImmediateMode)?;
                    self.write_memory(output_location as i32, a * b, get_param_mode(op, 2))?;
                    self.instruction_pointer += 4;
                },
                3 => {
                    // input
                    let output_location = self.load_param(&(self.instruction_pointer + 1), ParamMode::ImmediateMode)? as i32;
                    if let Some(value) = self.input_queue.pop_front() {
                        self.write_memory(output_location, value, get_param_mode(op, 0))?;
                        self.instruction_pointer += 2;
                    } else {
                        return Ok(IntcodeComputerState::WaitingForInput);
                    }
                },
                4 => {
                    // output
                    let output = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    self.instruction_pointer += 2;
                    self.output.push(output);
                    self.output_queue.push_back(output);
                    return Ok(IntcodeComputerState::OutputProduced(output));
                },
                5 => {
                    // jump if true
                    let input = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    let target = self.load_param(&(self.instruction_pointer + 2), get_param_mode(op, 1))?;
                    if input != 0 {
                        self.instruction_pointer = target as i32;
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                6 => {
                    // jump if false
                    let input = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    let target = self.load_param(&(self.instruction_pointer + 2), get_param_mode(op, 1))?;
                    if input == 0 {
                        self.instruction_pointer = target as i32;
                    } else {
                        self.instruction_pointer += 3;
                    }
                },
                7 => {
                    // less than
                    let a = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    let b = self.load_param(&(self.instruction_pointer + 2), get_param_mode(op, 1))?;
                    let output_location = self.load_param(&(self.instruction_pointer + 3), ParamMode::ImmediateMode)?;
                    self.write_memory(output_location as i32, if a < b { 1 } else { 0 }, get_param_mode(op, 2))?;
                    self.instruction_pointer += 4;
                },
                8 => {
                    // equals
                    let a = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))?;
                    let b = self.load_param(&(self.instruction_pointer + 2), get_param_mode(op, 1))?;
                    let output_location = self.load_param(&(self.instruction_pointer + 3), ParamMode::ImmediateMode)?;
                    self.write_memory(output_location as i32, if a == b { 1 } else { 0 }, get_param_mode(op, 2))?;
                    self.instruction_pointer += 4;
                },
                9 => {
                    // shift relative base
                    let a = self.load_param(&(self.instruction_pointer + 1), get_param_mode(op, 0))? as i32;
                    self.relative_base += a;
                    self.instruction_pointer += 2;
                }
                99 => {
                    return Ok(IntcodeComputerState::Halted);
                },
                _ => Err(format!("Unsupported operation {} at {}", get_op_code(op), self.instruction_pointer))?,
            }
        }
    }
}

fn load_input(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .split(",")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn get_op_code(op: &i64) -> i64 {
    op % 100
}

fn get_param_mode(op: &i64, param_index: u32) -> ParamMode {
    let mode = op / (10_i64.pow(param_index + 2)) % 10;
    match mode {
        0 => ParamMode::PositionMode,
        1 => ParamMode::ImmediateMode,
        2 => ParamMode::RelativeMode,
        _ => panic!("Unknown param mode"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_op_code_extracts() {
        assert_eq!(get_op_code(&1002), 02);
        assert_eq!(get_op_code(&2), 02);
    }

    #[test]
    fn get_param_mode_one() {
        assert_eq!(get_param_mode(&1002, 0), ParamMode::PositionMode);
        assert_eq!(get_param_mode(&1002, 1), ParamMode::ImmediateMode);
        assert_eq!(get_param_mode(&1002, 2), ParamMode::PositionMode);
    }

    #[test]
    fn test_computer_one() {
        let input = vec![1002, 4, 3, 4, 33];
        let mut computer = IntcodeComputer::new(input);
        computer.run().unwrap();
        assert_eq!(computer.dump_memory(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_computer_two() {
        let input = vec![1101, 100, -1, 4, 0];
        let mut computer = IntcodeComputer::new(input);
        computer.run().unwrap();
        assert_eq!(computer.dump_memory(), vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_computer_input() {
        let program = load_input("input/day_five.txt");
        let mut computer = IntcodeComputer::new(program);
        computer.provide_input(1);
        computer.run_ignore_output().unwrap();
        assert_eq!(computer.get_output(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 9006673]);
    }

    #[test]
    fn jump_if_true_yes_pos_mode() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut computer = IntcodeComputer::new(program);
        let mut output = -1;
        loop {
            match computer.run().unwrap() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(6),
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
            match computer.run().unwrap() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(0), 
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
            match computer.run().unwrap() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(6),
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
            match computer.run().unwrap() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(0),
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
            match computer.run().unwrap() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(res) => output = res,
                IntcodeComputerState::WaitingForInput => computer.provide_input(input.pop().unwrap())
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
