use std::fs;
use std::ops::Add;
use std::fmt;
use std::collections::{HashMap, HashSet};

fn get_input() -> String {
    fs::read_to_string("input/day_twelve.txt")
        .expect("Something went wrong reading the file")
}

fn parse_input(text: &str) -> Vec<Moon> {
    let mut moons = vec![];
    let text = text.replace("<", "")
            .replace(">", "")
            .replace("x", "")
            .replace("y", "")
            .replace("z", "")
            .replace("=", "");
    for line in text.lines() {
        let mut split = line.split(",");
        let x: i32 = split.next().unwrap().trim().parse().unwrap();
        let y: i32 = split.next().unwrap().trim().parse().unwrap();
        let z: i32 = split.next().unwrap().trim().parse().unwrap();

        moons.push(Moon::new(Vector::new(x, y, z)));
    }
    moons
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector {x, y, z}
    }

    fn sum_abs(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Moon {
    position: Vector,
    velocity: Vector
}

impl Moon {
    fn new(position: Vector) -> Moon {
        Moon {
            position,
            velocity: Vector::new(0, 0, 0),
        }
    }

    fn total_energy(&self) -> i32 {
        let potential_energy = self.position.sum_abs();
        let kinetic_energy = self.velocity.sum_abs();
        potential_energy * kinetic_energy
    }
}

impl fmt::Display for &Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=<x={}, y={}, z={}>, vel=<x= {}, y={}, z={}>",
                self.position.x,
                self.position.y,
                self.position.z,
                self.velocity.x,
                self.velocity.y,
                self.velocity.z
            )
    }
}

fn simulate_moons(moons: &Vec<Moon>, steps: i32) -> Vec<Moon> {
    let mut moons = moons.clone();
    for _ in 0..steps {
        let mut new_moons = moons.clone();
        for current_moon in new_moons.iter_mut() {
            // adjust velocities
            for other_moon in moons.iter() {
                if current_moon.position.x > other_moon.position.x {
                    current_moon.velocity.x -= 1
                } else if current_moon.position.x < other_moon.position.x {
                    current_moon.velocity.x += 1
                }
                if current_moon.position.y > other_moon.position.y {
                    current_moon.velocity.y -= 1
                } else if current_moon.position.y < other_moon.position.y {
                    current_moon.velocity.y += 1
                }
                if current_moon.position.z > other_moon.position.z {
                    current_moon.velocity.z -= 1
                } else if current_moon.position.z < other_moon.position.z {
                    current_moon.velocity.z += 1
                }
            }
            // adjust position
            current_moon.position = &current_moon.position + &current_moon.velocity;
        }
        moons = new_moons;
    }
    moons
}

fn simulate_axis_until_repeat(moons: &Vec<Moon>) -> i32 {
    let mut previous: HashMap<usize, HashMap<usize, HashSet<i32>>> = HashMap::new();
    let mut oscilation: HashMap<usize, HashMap<usize, Option<i32>>> = HashMap::new();
    let mut counter = 0;
    let mut moons = moons.clone();
    for i in 0..4 {
        let mut local_map: HashMap<usize, HashSet<i32>> = HashMap::new();
        let mut local_osc: HashMap<usize, Option<i32>> = HashMap::new();
        for o in 0..3 {
            local_map.insert(o, HashSet::new());
            local_osc.insert(o, None);
        }
        previous.insert(i, local_map);
        oscilation.insert(i, local_osc);
    }
    loop {
        counter += 1;
        let mut new_moons = moons.clone();
        for (i, current_moon) in new_moons.iter_mut().enumerate() {
            // adjust velocities
            for other_moon in moons.iter() {
                if current_moon.position.x > other_moon.position.x {
                    current_moon.velocity.x -= 1
                } else if current_moon.position.x < other_moon.position.x {
                    current_moon.velocity.x += 1
                }
                if current_moon.position.y > other_moon.position.y {
                    current_moon.velocity.y -= 1
                } else if current_moon.position.y < other_moon.position.y {
                    current_moon.velocity.y += 1
                }
                if current_moon.position.z > other_moon.position.z {
                    current_moon.velocity.z -= 1
                } else if current_moon.position.z < other_moon.position.z {
                    current_moon.velocity.z += 1
                }
            }
            // adjust position
            current_moon.position = &current_moon.position + &current_moon.velocity;
            if oscilation.get(&i).unwrap().get(&0).unwrap().is_none() {
                // let mut set = previous.get_mut(&i).unwrap().get_mut(&0).unwrap();
                // if 
            }
        }
        if counter % 1000000 == 0 {
            println!("{}", counter)
        }
        moons = new_moons;
    }
}

pub fn one() {
    let text = get_input();
    let moons = parse_input(&text);
    let moons = simulate_moons(&moons, 1000);
    let system_energy: i32 = moons.iter().map(|moon| moon.total_energy()).sum();
    println!("{}", system_energy)
}

// pub fn two() {
//     let moons = parse_input("<x=-8, y=-10, z=0>
//     <x=5, y=5, z=10>
//     <x=2, y=-7, z=3>
//     <x=9, y=-8, z=-3>");
//     // let iter = simulate_until_repeat(&moons);
//     // println!("{}", iter);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_test_1() {
        let moons = parse_input("<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>");
        let moons = simulate_moons(&moons, 10);
        let system_energy: i32 = moons.iter().map(|moon| moon.total_energy()).sum();
        assert_eq!(system_energy, 179);
    }

    #[test]
    fn day_12_test_2() {
        let moons = parse_input("<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>");
        let moons = simulate_moons(&moons, 100);
        let system_energy: i32 = moons.iter().map(|moon| moon.total_energy()).sum();
        assert_eq!(system_energy, 1940);
    }

    fn day_12_task_1() {
        let text = get_input();
        let moons = parse_input(&text);
        let moons = simulate_moons(&moons, 1000);
        let system_energy: i32 = moons.iter().map(|moon| moon.total_energy()).sum();
        assert_eq!(5517, system_energy)
    }
}