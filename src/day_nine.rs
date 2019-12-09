use crate::infinite_memory_intcomputer::*;

pub fn one() {
    let mut computer = IntcodeComputer::new_from_file("input/day_nine.txt");
    computer.provide_input(1);
    loop {
        match computer.run().unwrap() {
            IntcodeComputerState::Halted => break,
            IntcodeComputerState::OutputProduced(output) => println!("Output {}", output),
            IntcodeComputerState::WaitingForInput => panic!("Unexpected waiting for input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = IntcodeComputer::new(program.clone());
        let mut output_vec = vec![];
        while let Ok(IntcodeComputerState::OutputProduced(output)) = computer.run() {
            output_vec.push(output);
        }
        assert_eq!(output_vec, program);
    }

    #[test]
    fn large_number() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = IntcodeComputer::new(program.clone());
        let mut output_vec = vec![];
        while let Ok(IntcodeComputerState::OutputProduced(output)) = computer.run() {
            output_vec.push(output);
        }
        assert_eq!(output_vec, vec![1219070632396864]);
    }

    #[test]
    fn middle_number() {
        let program = vec![104, 1125899906842624, 99];
        let mut computer = IntcodeComputer::new(program.clone());
        let mut output_vec = vec![];
        while let Ok(IntcodeComputerState::OutputProduced(output)) = computer.run() {
            output_vec.push(output);
        }
        assert_eq!(output_vec, vec![1125899906842624]);
    }

    #[test]
    fn into_code_computer_self_test() {
        let mut computer = IntcodeComputer::new_from_file("input/day_nine.txt");
        computer.provide_input(1);
        loop {
            match computer.run().unwrap() {
                IntcodeComputerState::Halted => break,
                IntcodeComputerState::OutputProduced(output) => assert_eq!(output, 3100786347),
                IntcodeComputerState::WaitingForInput => panic!("Unexpected waiting for input"),
            }
        }
    }
}
