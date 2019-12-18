use crate::infinite_memory_intcomputer::{IntcodeComputer, IntcodeComputerState};
use std::collections::HashMap;
use std::error::*;

const BLACK: i32 = 0;
const WHITE: i32 = 1;

const LEFT: i64 = 0;
const RIGHT: i64 = 1;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Robot {
    computer: IntcodeComputer,
    map: HashMap<(i32, i32), i32>,
    direction: Direction,
    position: (i32, i32),
    painted_panels: i32,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            computer: IntcodeComputer::new_from_file("input/day_eleven.txt"),
            map: HashMap::new(),
            direction: Direction::Up,
            position: (0, 0),
            painted_panels: 0
        }
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let current_color = self.map.get(&self.position).unwrap_or(&BLACK);
            self.computer.provide_input(*current_color as i64);
            let paint_color = self.computer.run()?;
            let turn_direction = self.computer.run()?;
            if let (IntcodeComputerState::OutputProduced(paint_color),
                    IntcodeComputerState::OutputProduced(turn_direction)) =
                                    (paint_color, turn_direction) {
                if !self.map.contains_key(&self.position){
                    self.painted_panels+=1;
                }
                self.map.insert(self.position, paint_color as i32);
                // update direction
                if turn_direction == LEFT {
                    match self.direction {
                        Direction::Up => self.direction = Direction::Left,
                        Direction::Left => self.direction = Direction::Down,
                        Direction::Down => self.direction = Direction::Right,
                        Direction::Right => self.direction = Direction::Up,
                    }
                } else if turn_direction == RIGHT {
                    match self.direction {
                        Direction::Up => self.direction = Direction::Right,
                        Direction::Left => self.direction = Direction::Up,
                        Direction::Down => self.direction = Direction::Left,
                        Direction::Right => self.direction = Direction::Down,
                    }
                } else {
                    panic!("Wrong turn command {}", turn_direction);
                }
                // update position
                let (x, y) = self.position;
                match self.direction {
                    Direction::Up => self.position = (x, y-1),
                    Direction::Left => self.position = (x-1, y),
                    Direction::Down => self.position = (x, y+1),
                    Direction::Right => self.position = (x+1, y),
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}

pub fn one() {
    let mut robot = Robot::new();
    robot.run().unwrap();
    println!("{:?}", robot.painted_panels);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_task_1() {
        let mut robot = Robot::new();
        robot.run().unwrap();
        assert_eq!(2469, robot.painted_panels);
    }
}