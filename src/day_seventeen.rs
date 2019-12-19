use crate::infinite_memory_intcomputer::IntcodeComputer;
use std::collections::{HashSet, HashMap};

const SCAFFOLD: i64 = 35;
const OPEN: i64 = 46;
const NEW_LINE: i64 = 10;

fn is_scaffolding(input: &char) -> bool {
    "<>^>#".contains(*input)
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