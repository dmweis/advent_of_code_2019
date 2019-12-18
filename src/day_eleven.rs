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

pub fn two() {
    let mut robot = Robot::new();
    robot.map.insert((0, 0), WHITE);
    robot.run().unwrap();
    let mut picture = robot.map.clone();
    picture.insert((0, 0), 3);

    let mut top_left = (std::i32::MAX, std::i32::MAX);
    let mut bottom_right = (std::i32::MIN, std::i32::MIN);
    for point in picture.keys() {
        if point.0 < top_left.0 {
            top_left.0 = point.0
        }
        if point.1 < top_left.1 {
            top_left.1 = point.1
        }
        if point.0 > bottom_right.0 {
            bottom_right.0 = point.0
        }
        if point.1 > bottom_right.1 {
            bottom_right.1 = point.1
        }
    }
    println!("tl {:?}", top_left);
    println!("br {:?}", bottom_right);
    for x in top_left.0..(bottom_right.0+1) {
        for y in top_left.1..(bottom_right.1+1) {
            let pixel = picture.get(&(x, y)).unwrap_or(&BLACK);
            if pixel == &WHITE {
                print!("#");
            } else if pixel == &BLACK {
                print!(" ");
            } else {
                print!("O")
            }
        }
        println!("");
    }
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

    #[test]
    fn day_11_task_2() {
        let mut robot = Robot::new();
        robot.map.insert((0, 0), WHITE);
        robot.run().unwrap();
        assert_eq!(248, robot.painted_panels);
    }
}