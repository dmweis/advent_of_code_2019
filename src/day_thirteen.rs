use crate::infinite_memory_intcomputer::{IntcodeComputer, IntcodeComputerState::{OutputProduced, WaitingForInput}, ParamMode::PositionMode};
use std::collections::HashMap;
use std::error::*;
use std::{thread, time};

const EMPTY: i32 = 0;
const WALL: i32 = 1;
const BLOCK: i32 = 2;
const HORIZONTAL_PADDLE: i32 = 3;
const BALL: i32 = 4;


struct ArcadeCAbinet {
    computer: IntcodeComputer,
    display: HashMap<(i32, i32), i32>,
    score: i32,
}

impl ArcadeCAbinet {
    fn new() -> ArcadeCAbinet {
        ArcadeCAbinet {
            computer: IntcodeComputer::new_from_file("input/day_thirteen.txt"),
            display: HashMap::new(),
            score: 0,
        }
    }

    fn insert_coins(&mut self) -> Result<(), Box<dyn Error>> {
        self.computer.write_memory(0, 2, PositionMode)
    }

    fn input(&mut self, input: i32) {
        self.computer.provide_input(input as i64);
    }

    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let x = self.computer.run()?;
            let y = self.computer.run()?;
            let pixel = self.computer.run()?;
            if let (OutputProduced(x), OutputProduced(y), OutputProduced(pixel)) = (x, y, pixel) {
                self.display.insert((x as i32, y as i32), pixel as i32);
            } else {
                return Ok(())
            }
        }
    }

    fn run_game(&mut self) -> Result<bool, Box<dyn Error>> {
        loop {
            let x = self.computer.run()?;
            let y = self.computer.run()?;
            let pixel = self.computer.run()?;
            if let (OutputProduced(x), OutputProduced(y), OutputProduced(pixel)) = (&x, &y, &pixel) {
                // check score
                if x == &-1 && y == &0 {
                    self.score = *pixel as i32;
                } else {
                    self.display.insert((*x as i32, *y as i32), *pixel as i32);
                }
            } else if let WaitingForInput = pixel {
                return Ok(true)
            } else {
                return Ok(false)
            }
        }
    }
}

pub fn one() {
    let mut cabinet = ArcadeCAbinet::new();
    cabinet.run().unwrap();
    let display = cabinet.display.clone();
    let mut block_count = 0;
    for (_, pixel) in &display {
        if pixel == &BLOCK {
            block_count += 1;
        }
    }
    println!("{}", block_count);
}

fn render_frame(frame: &HashMap<(i32, i32), i32>) -> ((i32, i32), (i32, i32), String, i32) {
    let mut top_left = (std::i32::MAX, std::i32::MAX);
    let mut bottom_right = (std::i32::MIN, std::i32::MIN);
    for point in frame.keys() {
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

    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    let mut line_count = 0;
    let mut buffer = String::new();
    for y in top_left.1..(bottom_right.1+1) {
        for x in top_left.0..(bottom_right.0+1) {
            let pixel = frame.get(&(x, y)).unwrap_or(&EMPTY);
            match pixel {
                &EMPTY => buffer.push_str(" "),
                &WALL => buffer.push_str("|"),
                &BLOCK => buffer.push_str("❤️"),
                &HORIZONTAL_PADDLE => {
                    buffer.push_str("_");
                    paddle = (x, y);
                },
                &BALL => {
                    buffer.push_str("O");
                    ball = (x, y);
                },
                _ => panic!("unknown symbol"),
            }
        }
        buffer.push_str("               \n");
        line_count+=1;
    }
    // println!("{}", buffer);
    (ball, paddle, buffer, line_count)
}

pub fn two() {
    let mut cabinet = ArcadeCAbinet::new();
    cabinet.insert_coins().unwrap();
    let mut prev_lines = None;
    while cabinet.run_game().unwrap() {
        if let Some(line_count) = prev_lines {
            println!("\x1b[{}F", line_count+2);
        }
        let (ball, paddle, frame, lines) = render_frame(&cabinet.display);
        prev_lines = Some(lines);
        println!("{}", frame);
        thread::sleep(time::Duration::from_millis(1));
        // Smarty AI
        if ball.0 > paddle.0 {
            cabinet.input(1);
        } else if ball.0 < paddle.0 {
            cabinet.input(-1)
        } else {
            cabinet.input(0)
        }
    }
    println!("your score is {}", cabinet.score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_13_task_1() {
        let mut cabinet = ArcadeCAbinet::new();
        cabinet.run().unwrap();
        let display = cabinet.display.clone();
        let mut block_count = 0;
        for (_, pixel) in &display {
            if pixel == &BLOCK {
                block_count += 1;
            }
        }
        assert_eq!(258, block_count);
    }

    #[test]
    fn day_13_task_2() {
        let mut cabinet = ArcadeCAbinet::new();
        cabinet.insert_coins().unwrap();
        while cabinet.run_game().unwrap() {
            let (ball, paddle, _, _) = render_frame(&cabinet.display);
            // Smarty AI
            if ball.0 > paddle.0 {
                cabinet.input(1);
            } else if ball.0 < paddle.0 {
                cabinet.input(-1)
            } else {
                cabinet.input(0)
            }
        }
        assert_eq!(12765, cabinet.score);
    }
}