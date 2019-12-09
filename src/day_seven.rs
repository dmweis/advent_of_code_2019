use crate::intcode_computer::*;
use crate::infinite_memory_intcomputer::*;
use itertools::*;
use std::error;

fn run_thruster_computer(
    phase_setting: i32,
    input_signal: i32,
    program: &Vec<i32>,
) -> Result<i32, Box<dyn error::Error>> {
    let mut program = program.clone();
    let inputs = vec![phase_setting, input_signal];
    let output = process_intcode(&mut program, Some(inputs));
    let res = output.first().ok_or("Thruster didn't produce a result")?;
    Ok(*res)
}

fn run_all_thursters(
    phase_sequence: Vec<i32>,
    program: &Vec<i32>,
) -> Result<i32, Box<dyn error::Error>> {
    let mut input_signal = 0;
    for phase_setting in phase_sequence {
        input_signal = run_thruster_computer(phase_setting, input_signal, program)?;
    }
    Ok(input_signal)
}

pub fn one() {
    let program = load_input("input/day_seven.txt");
    let max = (0..5)
        .permutations(5)
        .map(|s| run_all_thursters(s, &program))
        .filter_map(Result::ok)
        .max();
    println!("{:?}", max);
}

fn run_all_thursters_together(
    phase_sequence: Vec<i64>,
    program: Vec<i64>,
) -> Result<i64, Box<dyn error::Error>> {
    let mut computers = vec![];
    for phase in phase_sequence {
        let mut computer = IntcodeComputer::new(program.clone());
        computer.run()?;
        computer.provide_input(phase);
        computers.push(computer);
    }
    let mut last_output = Some(0);
    let mut done = false;
    while !done {
        for computer in &mut computers {
            loop {
                match computer.run()? {
                    IntcodeComputerState::WaitingForInput => {
                        computer.provide_input(last_output.unwrap());
                        last_output = None;
                    },
                    IntcodeComputerState::OutputProduced(output) => {
                        last_output = Some(output);
                        break;
                    },
                    IntcodeComputerState::Halted => {
                        done = true;
                        break;
                    },
                }
            }
            if done {
                break;
            }
        }
    }
    Ok(last_output.unwrap())
}

pub fn two() {
    let program: Vec<i64> = load_input("input/day_seven.txt").iter().map(|i| *i as i64).collect();
    let max = (5..10)
        .permutations(5)
        .map(|s| run_all_thursters_together(s, program.clone()))
        .filter_map(Result::ok)
        .max();
    println!("{:?}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let max = run_all_thursters(vec![43210], &program).unwrap();
        assert_eq!(max, 43210);
    }

    #[test]
    fn test_2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let max = run_all_thursters(vec![0, 1, 2, 3, 4], &program).unwrap();
        assert_eq!(max, 54321);
    }

    #[test]
    fn test_3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let max = run_all_thursters(vec![1, 0, 4, 3, 2], &program).unwrap();
        assert_eq!(max, 65210);
    }

    #[test]
    fn day_7_task_1() {
        let program = load_input("input/day_seven.txt");
        let max = (0..5)
            .permutations(5)
            .map(|s| run_all_thursters(s, &program))
            .filter_map(Result::ok)
            .max()
            .unwrap();
        assert_eq!(max, 87138);
    }

    #[test]
    fn computer_2_test_1() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let output = run_all_thursters_together(vec![9, 8, 7, 6, 5], program).unwrap();
        assert_eq!(output, 139629729);
    }

    #[test]
    fn computer_2_test_2() {
        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let output = run_all_thursters_together(vec![9, 7, 8, 5, 6], program).unwrap();
        assert_eq!(output, 18216);
    }

    #[test]
    fn day_7_task_2() {
        let program: Vec<i64> = load_input("input/day_seven.txt").iter().map(|i| *i as i64).collect();
        let max = (5..10)
            .permutations(5)
            .map(|s| run_all_thursters_together(s, program.clone()))
            .filter_map(Result::ok)
            .max()
            .unwrap();
        assert_eq!(max, 17279674);
    }
}
