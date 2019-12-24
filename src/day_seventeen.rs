use crate::infinite_memory_intcomputer::{IntcodeComputer, ParamMode};
use std::collections::{HashSet, HashMap,};

const SCAFFOLD: i64 = 35;
const OPEN: i64 = 46;
const NEW_LINE: i64 = 10;

fn is_scaffolding(input: &char) -> bool {
    "<>^v#".contains(*input)
}

fn is_intersection(pos: &(i32, i32), map: &HashSet<(i32, i32)>) -> bool {
    map.contains(pos) &&
    map.contains(&(pos.0-1, pos.1)) &&
    map.contains(&(pos.0+1, pos.1)) &&
    map.contains(&(pos.0, pos.1-1)) &&
    map.contains(&(pos.0, pos.1+1))
}

pub fn one() {
    let mut computer = IntcodeComputer::new_from_file("input/day_seventeen.txt");
    computer.run_ignore_output().unwrap();
    let data = computer.get_output();
    let mut scaffolding_map = HashSet::new();
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut width = 0;
    for point in data {
        let pixel = point as u8 as char;
        print!("{}", pixel);
        if point == NEW_LINE {
            y+=1;
            if x > width {
                width = x;
            }
            x=0;
        } else {
            map.insert((x, y), pixel);
            if is_scaffolding(&pixel) {
                scaffolding_map.insert((x, y));
            }
            x+=1;
        }
    }
    let height = y - 1;
    println!("width {} height {}", width, height);
    // find intersections
    let mut intersections = HashSet::new();
    for y in 0..(height+1) {
        for x in 0..(width+1) {
            if is_intersection(&(x, y), &scaffolding_map) {
                map.insert((x, y), '╳');
                intersections.insert((x, y));
            }
        }
    }
    for y in 0..height {
        for x in 0..width {
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!("");
    }
    let mut sum_of_alignments = 0;
    for (x, y) in intersections {
        println!("inter at <{} {}> mul {}", x, y, x * y);
        sum_of_alignments += x * y;
    }
    println!("Sum of alignments {}", sum_of_alignments);
}

const LEFT: char = '<';
const RIGHT: char = '>';
const UP: char = '^';
const DOWN: char = 'v';

#[derive(Clone)]
enum RobotDirection {
    Left,
    Right,
    Up,
    Down
}

impl RobotDirection {
    fn from_char(symbol: &char) -> RobotDirection {
        match symbol {
            &LEFT => RobotDirection::Left,
            &RIGHT => RobotDirection::Right,
            &UP => RobotDirection::Up,
            &DOWN => RobotDirection::Down,
            _ => panic!("Unknown direction")
        }
    }

    fn to_char(&self) -> char {
        match self {
            RobotDirection::Up => UP,
            RobotDirection::Down => DOWN,
            RobotDirection::Left => LEFT,
            RobotDirection::Right => RIGHT,
        }
    }

    fn move_point(&self, point: &(i32, i32)) -> (i32, i32) {
        let (x, y) = point;
        match self {
            RobotDirection::Up => (*x, y-1),
            RobotDirection::Down => (*x, y+1),
            RobotDirection::Left => (x-1, *y),
            RobotDirection::Right => (x+1, *y),
        }
    }

    fn get_turn_directions(&self) -> (RobotDirection, RobotDirection) {
        match self {
            RobotDirection::Up => (RobotDirection::Left, RobotDirection::Right),
            RobotDirection::Down => (RobotDirection::Right, RobotDirection::Left),
            RobotDirection::Left => (RobotDirection::Down, RobotDirection::Up),
            RobotDirection::Right => (RobotDirection::Up, RobotDirection::Down),
        }
    }
}

struct ScaffoldingMap {
    points: HashSet<(i32, i32)>,
    robot_start: (i32, i32),
    robot_direction: RobotDirection,
    width: i32,
    height: i32,
}

fn is_robot(input: &char) -> bool {
    "<>^v".contains(|p| &p == input)
}

impl ScaffoldingMap {
    fn new(input: &str) -> ScaffoldingMap {
        let mut points = HashSet::new();
        let mut robot_start = (0, 0);
        let mut robot_direction = RobotDirection::Up;
        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            height = height.max(y as i32);
            for (x, point) in line.chars().enumerate() {
                width = width.max(x as i32 + 1);
                if point as i64 == SCAFFOLD {
                    points.insert((x as i32, y as i32));
                } else if is_robot(&point) {
                    robot_start = (x as i32, y as i32);
                    robot_direction = RobotDirection::from_char(&point);
                }
            }
        }
        ScaffoldingMap {
            points,
            robot_start,
            robot_direction,
            width,
            height,
        }
    }

    fn render(&self, erase: bool) {
        if erase {
            println!("\x1b[{}F", self.height + 1);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let point = if self.robot_start == (x, y) {
                    self.robot_direction.to_char()
                } else if self.points.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", point);
            }
            println!("");
        }
    }

    fn render_with_robot(&self, position: &(i32, i32), direction: &RobotDirection) {
        println!("\x1b[{}F", self.height + 1);
        for y in 0..self.height {
            for x in 0..self.width {
                let point = if position == &(x, y) {
                    direction.to_char()
                } else if self.points.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", point);
            }
            println!("");
        }
    }

    fn search(&self) -> Vec<String> {
        let mut robot_direction = self.robot_direction.clone();
        let mut robot_position = self.robot_start.clone();
        let mut path = vec![];
        let mut forward_moves = 0;
        loop {
            self.render_with_robot(&robot_position, &robot_direction);
            // sleep(Duration::from_millis(5));
            let next_point = robot_direction.move_point(&robot_position);
            if self.points.contains(&next_point) {
                robot_position = next_point;
                forward_moves += 1;
                continue;
            }
            let (left, right) = robot_direction.get_turn_directions();
            let next_left = left.move_point(&robot_position);
            if self.points.contains(&next_left) {
                robot_position = next_left;
                robot_direction = left.clone();
                if forward_moves > 0 {
                    path.push(forward_moves.to_string());
                }
                path.push("L".to_string());
                forward_moves = 0;
                continue;
            }
            let next_right = right.move_point(&robot_position);
            if self.points.contains(&next_right) {
                robot_position = next_right;
                robot_direction = right.clone();
                if forward_moves > 0 {
                    path.push(forward_moves.to_string());
                }
                path.push("L".to_string());
                forward_moves = 0;
                continue;
            }
            path.push(forward_moves.to_string());
            break;
        }
        path
    }
}

fn build_path(path: &Vec<String>) -> Vec<(String, usize)> {
    let path_string = path.join(",");
    let mut repeating = HashSet::new();
    for i in 2..5 {
        let strings = path
            .windows(i)
            .filter_map(|slice| {
                let sub = slice.join(",");
                if sub.len() < 19 {
                    Some(sub)
                } else {
                    None
                }
            });
        for slice in strings {
            repeating.insert(slice);
        }
    }
    let mut repeating_paths: Vec<(String, usize)> = repeating
                                        .iter()
                                        .map(|s| (s.to_owned(), path_string.matches(s).count()))
                                        .collect();
    repeating_paths.sort_by_key(|(_, l)| *l);
    repeating_paths.reverse();
    repeating_paths
}

pub fn two() {
    let mut computer = IntcodeComputer::new_from_file("input/day_seventeen.txt");
    computer.write_memory(0, 2, ParamMode::PositionMode).unwrap();
    computer.run_ignore_output().unwrap();


    let output = computer.pop_output();
    
    let map: String = output.iter().map(|c| *c as u8 as char).take_while(|c| !c.is_alphabetic()).collect();
    let scaffolding_map = ScaffoldingMap::new(&map);
    scaffolding_map.render(false);
    let path = scaffolding_map.search();
    let text: String = path.iter().map(|s| s.to_owned()).collect();
    println!("{:?}", text);
    println!("{:?}", path);
    println!("\n{:?}", build_path(&path));
    // println!("{}", text);
    // println!("{}", text.matches("L7L11L11").count());
    // println!("{}", text.replace("L7L11L11", " "));
    // println!("{}", text.replace("L7L11L11", " ").matches("L7").count());
    // println!("{}", text.replace("L7", " "));
    // println!("{}", text.matches("L9").count());
    // println!("{}", text.replace("L9", " "));
    // println!("{}", text.replace("L7L11L11", "A").replace("L9", "B").replace("L7", "C"));

    // let text: String = output.iter().map(|c| *c as u8 as char).collect();
    // println!("\n\n{}", text);

    computer.provide_input_iter("A\n".chars().map(|c| c as i64));

    computer.run_ignore_output().unwrap();
    // let output = computer.pop_output();
    // let text: String = output.iter().map(|c| *c as u8 as char).collect();
    // println!("2 {}", text);

    computer.provide_input_iter("0\n0\n0\ny\n".chars().map(|c| c as i64));

    computer.run_ignore_output().unwrap();
    // let output = computer.pop_output();
    // let text: String = output.iter().map(|c| *c as u8 as char).collect();
    // println!("3 {}", text);

    // println!("4 {}", computer.pop_output().iter().last().unwrap());
}