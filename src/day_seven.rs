use std::error;
use itertools::*;
use crate::intcode_computer::*;

fn run_thruster_computer(phase_setting: i32, input_signal: i32, program: &Vec<i32>) -> Result<i32, Box<dyn error::Error>> {
    let mut program = program.clone();
    let inputs = vec![phase_setting, input_signal];
    let output = process_intcode(&mut program, Some(inputs));
    let res = output.first().ok_or("Thruster didn't produce a result")?;
    Ok(*res)
}

fn run_all_thursters(phase_sequence: Vec<i32>, program: &Vec<i32>) -> Result<i32, Box<dyn error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let max = run_all_thursters(vec![43210], &program).unwrap();
        assert_eq!(max, 43210);
    }

    #[test]
    fn test_2() {
        let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let max = run_all_thursters(vec![0,1,2,3,4], &program).unwrap();
        assert_eq!(max, 54321);
    }

    #[test]
    fn test_3() {
        let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let max = run_all_thursters(vec![1,0,4,3,2], &program).unwrap();
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
}
