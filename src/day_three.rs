use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn get_input() -> Vec<Vec<String>> {
    let input = fs::read_to_string("input/day_three.txt")
        .expect("Something went wrong reading the file");
    let lines = input.lines();
    let mut results = vec![];
    for line in lines {
        let operations = line.split(",").map(|s| s.to_owned()).collect();
        results.push(operations);
    }
    results
}

fn translate_to_points(directions: &Vec<String>) -> Vec<((i32, i32), i32)> {
    let mut position = (0, 0);
    let mut path = vec![];
    let mut step_count = 1;
    for operation in directions {
        let direction = &operation[0..1];
        let distance = operation[1..].parse::<i32>().unwrap() + 1;
        match direction {
            "L" => {
                for i in 1..distance {
                    path.push(((position.0-i, position.1), step_count));
                    step_count+=1;
                }
                position = path.last().unwrap().0;
            }
            "D" => {
                for i in 1..distance {
                    path.push(((position.0, position.1+i), step_count));
                    step_count+=1;
                }
                position = path.last().unwrap().0;
            },
            "R" => {
                for i in 1..distance {
                    path.push(((position.0+i, position.1), step_count));
                    step_count+=1;
                }
                position = path.last().unwrap().0;
            },
            "U" => {
                for i in 1..distance {
                    path.push(((position.0, position.1-i), step_count));
                    step_count+=1;
                }
                position = path.last().unwrap().0;
            },
            _ => panic!("Error while parsing. Unknown direction {}", direction),
        }
    }
    path
}

pub fn one() {
    let data = get_input();
    let points_a: HashSet<_> = translate_to_points(&data[0]).iter().map(|a| a.0).collect();
    let points_b: HashSet<_> = translate_to_points(&data[1]).iter().map(|a| a.0).collect();
    let intersect = points_a.intersection(&points_b);

    let closest = intersect.min_by_key(|(a, b)| a.abs() + b.abs()).unwrap();
    println!("closest is {:?} with distance {}", closest, closest.0.abs() + closest.1.abs());
}

pub fn two() {
    let data = get_input();
    let points_a: HashMap<(i32, i32), i32> = translate_to_points(&data[0]).iter().cloned().collect();
    let points_b: HashMap<(i32, i32), i32> = translate_to_points(&data[1]).iter().cloned().collect();
    let set_a = points_a.keys().collect::<HashSet<_>>();
    let set_b = points_b.keys().collect::<HashSet<_>>();
    let intersect = set_a.intersection(&set_b);

    let closest = intersect.min_by_key(|point| points_a[point].abs() + points_b[point].abs()).unwrap();
    println!("closest is {:?} with distance {} step distance {}",
            closest,
            closest.0.abs() + closest.1.abs(),
            points_a[closest].abs() + points_b[closest].abs());
}
