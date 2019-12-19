use crate::infinite_memory_intcomputer::{IntcodeComputer, IntcodeComputerState::OutputProduced};
use std::collections::{HashMap, VecDeque, HashSet};
use std::error::*;
use std::{thread, time};

const HIT_WALL: i64 = 0;
const MOVED: i64 = 1;
const MOVED_INTO_OXYGEN: i64 = 2;

const WALL: i32 = 0;
const SPACE: i32 = 1;
const OXYGEN: i32 = 2;
const ROBOT: i32 = 3;
const UNKNOWN: i32 = 4;
const SOLUTION_PATH: i32 = 5;

const NORTH: i32 = 1;
const SOUTH: i32 = 2;
const WEST: i32 = 3;
const EAST: i32 = 4;

fn opposite_direction(direction: &i32) -> i32 {
    match direction {
        &NORTH => SOUTH,
        &SOUTH => NORTH,
        &WEST => EAST,
        &EAST => WEST,
        _ => panic!("unknown direction")
    }
}

fn shift_position(position: &(i32, i32), direction: &i32) -> (i32, i32) {
    match direction {
        &NORTH => (position.0, position.1-1),
        &SOUTH => (position.0, position.1+1),
        &WEST => (position.0+1, position.1),
        &EAST => (position.0-1, position.1),
        _ => panic!("unknown direction")
    }
}

struct Map {
    points: HashMap<(i32, i32), i32>,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
    last_drawn_lines: Option<i32>,
}

impl Map {
    fn new() -> Map {
        Map {
            points: HashMap::new(),
            top_left: (std::i32::MAX, std::i32::MAX),
            bottom_right: (std::i32::MIN, std::i32::MIN),
            last_drawn_lines: None,
        }
    }

    fn add_point(&mut self, point: (i32, i32), tile: i32) -> Result<(), Box<dyn Error>> {
        if point.0 < self.top_left.0 {
            self.top_left.0 = point.0
        }
        if point.1 < self.top_left.1 {
            self.top_left.1 = point.1
        }
        if point.0 > self.bottom_right.0 {
            self.bottom_right.0 = point.0
        }
        if point.1 > self.bottom_right.1 {
            self.bottom_right.1 = point.1
        }
        if !self.points.insert(point, tile).is_none() {
            Err("Point already in map")?;
        }
        Ok(())
    }

    fn mark_path(&mut self, path: Vec<(i32, i32)>) {
        for element in path.iter().skip(1) {
            self.points.insert(element.clone(), SOLUTION_PATH);
            self.render();
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn get_point(&self, point: &(i32, i32)) -> i32 {
        *self.points.get(point).unwrap_or(&UNKNOWN)
    }

    fn get_oxygen(&self) -> Option<(i32, i32)> {
        for (key, value) in &self.points {
            if value == &OXYGEN {
                return Some(key.clone())
            }
        }
        None
    }

    fn contains(&self, point: &(i32, i32)) -> bool {
        self.points.contains_key(point)
    }

    fn render(&mut self) {
        if let Some(lines) = self.last_drawn_lines {
            println!("\x1b[{}F", lines + 1);
        }
        let mut lines = 0;
        for y in self.top_left.1..(self.bottom_right.1+1) {
            for x in self.top_left.0..(self.bottom_right.0+1) {
                let pixel = self.points.get(&(x, y)).unwrap_or(&UNKNOWN);
                match pixel {
                    &SPACE => print!(" "),
                    &WALL => print!("█"),
                    &OXYGEN => print!("╳"),
                    &UNKNOWN => print!("▒"),
                    &ROBOT => print!("O"),
                    &SOLUTION_PATH => print!("."),
                    _ => panic!("unknown pixel")
                }
            }
            lines+=1;
            println!("");
        }
        self.last_drawn_lines = Some(lines);
    }

    fn render_with_air(&self, air: &HashSet<(i32, i32)>, steps: &i32) {
        if let Some(lines) = self.last_drawn_lines {
            println!("\x1b[{}F", lines + 1);
        }
        for y in self.top_left.1..(self.bottom_right.1+1) {
            for x in self.top_left.0..(self.bottom_right.0+1) {
                if air.contains(&(x, y)) {
                    print!("#");
                    continue;
                }
                let pixel = self.points.get(&(x, y)).unwrap_or(&UNKNOWN);
                match pixel {
                    &SPACE => print!(" "),
                    &WALL => print!("█"),
                    &OXYGEN => print!("╳"),
                    &UNKNOWN => print!("▒"),
                    &ROBOT => print!("O"),
                    &SOLUTION_PATH => print!("."),
                    _ => panic!("unknown pixel")
                }
            }
            println!("");
        }
        print!("Steps: {}", steps);
    }
}

struct Robot {
    computer: IntcodeComputer,
    map: Map,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            computer: IntcodeComputer::new_from_file("input/day_fifteen.txt"),
            map: Map::new(),
        }
    }

    fn explore(&mut self) -> Result<(), Box<dyn Error>>{
        self.map.add_point((0, 0), ROBOT)?;
        self.explore_rec((0, 0))?;
        Ok(())
    }

    fn explore_rec(&mut self, position: (i32, i32)) -> Result<(), Box<dyn Error>> {
        for direction in vec![NORTH, WEST,SOUTH, EAST] {
            // println!("{}", direction);
            self.map.render();
            let new_pos = shift_position(&position, &direction);
            if self.map.contains(&new_pos) {
                continue;
            }
            self.computer.provide_input(direction as i64);
            let result = self.computer.run()?;
            if let OutputProduced(output) = &result {
                match output {
                    &HIT_WALL => {
                        self.map.add_point(new_pos, WALL)?;
                    }
                    &MOVED => {
                        self.map.add_point(new_pos.clone(), SPACE)?;
                        self.explore_rec(new_pos)?;
                        self.computer.provide_input(opposite_direction(&direction) as i64);
                        if let OutputProduced(res) = &self.computer.run()? {
                            if res != &(SPACE as i64) {
                                panic!("Ehm");
                            }
                        }
                    }
                    &MOVED_INTO_OXYGEN => {
                        self.map.add_point(new_pos.clone(), OXYGEN)?;
                        self.explore_rec(new_pos)?;
                        self.computer.provide_input(opposite_direction(&direction) as i64);
                        if let OutputProduced(res) = &self.computer.run()? {
                            if res != &(SPACE as i64) {
                                panic!("Ehm");
                            }
                        }
                    }
                    _ => panic!("Unknown response from computer")
                }
            } else {
                panic!("Computer mis-behaving");
            }
        }
        Ok(())
    }

    fn shortest_path(&self) -> Result<Vec<(i32, i32)>, ()>{
        let mut visited = HashSet::new();
        let mut paths = VecDeque::new();
        paths.push_front(vec![(0, 0)]);
        while let Some(path) = &mut paths.pop_back() {
            let end = path.pop().unwrap();
            if visited.contains(&end) {
                continue;
            }
            visited.insert(end.clone());
            for direction in vec![NORTH, SOUTH, WEST, EAST] {
                let new_pos = shift_position(&end, &direction);
                let point = self.map.get_point(&new_pos);
                if point == OXYGEN {
                    path.push(end);
                    // path.push(new_pos);
                    return Ok(path.clone());
                } else if point == SPACE {
                    let mut new_path = path.clone();
                    new_path.push(end.clone());
                    new_path.push(new_pos.clone());
                    paths.push_front(new_path);
                }
            }
        }
        Err(())
    }

    fn fill_room(&self) -> i32 {
        let mut visited = HashSet::new();
        let mut seen = VecDeque::new();
        let mut max_counter = 0;

        let oxygen_pos = self.map.get_oxygen().unwrap();
        seen.push_front((oxygen_pos, -1));
        while let Some((position, counter)) = seen.pop_back() {
            if counter > max_counter {
                max_counter = counter;
                self.map.render_with_air(&visited, &max_counter);
                thread::sleep(time::Duration::from_millis(50));
            }
            if visited.contains(&position) {
                continue;
            }
            visited.insert(position.clone());
            for direction in vec![NORTH, SOUTH, WEST, EAST] {
                let new_pos = shift_position(&position, &direction);
                let point = self.map.get_point(&new_pos);
                if point != WALL && point != UNKNOWN  {
                    seen.push_front((new_pos, counter+1));
                }
            }
        }
        max_counter
    }
}

pub fn one() {
    let mut robot = Robot::new();
    robot.explore().unwrap();
    robot.map.render();
    let path = robot.shortest_path().unwrap();
    let path_len = path.len();
    robot.map.mark_path(path);
    let time_to_fill = robot.fill_room();
    println!("\nPath length is {}", path_len);
    println!("time to fill is {}", time_to_fill);
}

#[cfg(test)]
mod tests {
    use super::*;

}