use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};
use std::time::Instant;

fn get_input() -> String {
    fs::read_to_string("input/day_twenty.txt")
        .expect("Something went wrong reading the file")
}

const WALL: char = '#';
const EMPTY: char = '.';
const NOTHING: char = ' ';
const PORTAL: char = '*';

struct Map {
    points: HashMap<(i32, i32), char>,
    portals: HashMap<(i32, i32), (i32, i32)>,
    start: Option<(i32, i32)>,
    end: Option<(i32, i32)>,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
    last_drawn_lines: Option<i32>,
}

impl Map {
    fn new() -> Map {
        Map {
            points: HashMap::new(),
            portals: HashMap::new(),
            start: None,
            end: None,
            top_left: (std::i32::MAX, std::i32::MAX),
            bottom_right: (std::i32::MIN, std::i32::MIN),
            last_drawn_lines: None,
        }
    }

    fn insert(&mut self, point: (i32, i32), tile: char) {
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
        self.points.insert(point, tile);
    }

    fn get_point(&self, point: &(i32, i32)) -> char {
        *self.points.get(point).unwrap_or(&NOTHING)
    }

    fn contains(&self, point: &(i32, i32)) -> bool {
        self.points.contains_key(point)
    }

    fn render(&mut self) {
        // if let Some(lines) = self.last_drawn_lines {
        //     println!("\x1b[{}F", lines);
        // }
        let mut lines = 0;
        for y in self.top_left.1..(self.bottom_right.1+1) {
            for x in self.top_left.0..(self.bottom_right.0+1) {
                let pixel = self.points.get(&(x, y)).unwrap_or(&NOTHING);
                let pixel = if pixel == &PORTAL {
                    if self.is_edge_portal(&(x, y)) {
                        '1'
                    } else {
                        '2'
                    }
                } else {
                    *pixel
                };
                print!("{}", pixel);
            }
            lines+=1;
            println!("");
        }
        self.last_drawn_lines = Some(lines);
    }

    fn render_with_path(&mut self, path: &Vec<(i32, i32)>) {
        let path: HashSet<(i32, i32)> = path.iter().map(|e| e.clone()).collect();
        let mut lines = 0;
        for y in self.top_left.1..(self.bottom_right.1+1) {
            for x in self.top_left.0..(self.bottom_right.0+1) {
                let pixel = if path.contains(&(x, y)) {
                    &'x'
                } else {
                    self.points.get(&(x, y)).unwrap_or(&NOTHING)
                };
                print!("{}", pixel);
            }
            lines+=1;
            println!("");
        }
        self.last_drawn_lines = Some(lines);
    }

    fn scan_portals(&mut self) {
        let mut portal_map = HashMap::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get_point(&(x, y)).is_ascii_uppercase() {
                    if self.get_point(&(x, y+1)).is_ascii_uppercase() {
                        // vertical
                        let a = self.get_point(&(x, y));
                        let b = self.get_point(&(x, y+1));
                        let mut key = vec![a, b];
                        key.sort();
                        if self.get_point(&(x, y-1)) == EMPTY {
                            // if empty above
                            if let Some(other_portal) = portal_map.insert(key, (x, y)) {
                                self.portals.insert(other_portal, (x, y));
                                self.portals.insert((x, y), other_portal);
                            }
                        } else {
                            // if empty bellow
                            if let Some(other_portal) = portal_map.insert(key, (x, y+1)) {
                                self.portals.insert(other_portal, (x, y+1));
                                self.portals.insert((x, y+1), other_portal);
                            }
                        }
                    } else if self.get_point(&(x+1, y)).is_ascii_uppercase() {
                        // horizontal
                        let a = self.get_point(&(x, y));
                        let b = self.get_point(&(x+1, y));
                        let mut key = vec![a, b];
                        key.sort();
                        if self.get_point(&(x-1, y)) == EMPTY {
                            // if empty left
                            if let Some(other_portal) = portal_map.insert(key, (x, y)) {
                                self.portals.insert(other_portal, (x, y));
                                self.portals.insert((x, y), other_portal);
                            }
                        } else {
                            // if empty right
                            if let Some(other_portal) = portal_map.insert(key, (x+1, y)) {
                                self.portals.insert(other_portal, (x+1, y));
                                self.portals.insert((x+1, y), other_portal);
                            }
                        }
                    }
                }
            }
        }
        // find start and end
        self.start = Some(portal_map.get(&vec!['A','A']).unwrap().clone());
        self.end = Some(portal_map.get(&vec!['Z','Z']).unwrap().clone());
    }

    fn mark_portals(&mut self) {
        for (source, target) in &self.portals {
            self.points.insert(target.clone(), PORTAL);
            self.points.insert(source.clone(), PORTAL);
        }
    }

    fn bfs(&self) -> Option<Vec<(i32, i32)>> {
        let mut visited = HashSet::new();
        let mut paths = VecDeque::new();
        paths.push_back(vec![self.start.unwrap().clone()]);
        while let Some(path) = paths.pop_front() {
            let current = path.iter().last().unwrap().clone();
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            for target in vec![
                (current.0+1, current.1),
                (current.0-1, current.1),
                (current.0, current.1+1),
                (current.0, current.1-1)
            ] {
                if target == self.end.unwrap() {
                    return Some(path.clone());
                }
                let target_point = self.get_point(&target);
                if target_point == EMPTY {
                    let mut new_path = path.clone();
                    new_path.push(target);
                    paths.push_back(new_path);
                } else if target_point == PORTAL {
                    let portal_exit = *self.portals.get(&target).unwrap();
                    for portal_exit_point in vec![
                        (portal_exit.0+1, portal_exit.1),
                        (portal_exit.0-1, portal_exit.1),
                        (portal_exit.0, portal_exit.1+1),
                        (portal_exit.0, portal_exit.1-1)
                    ] {
                        if self.get_point(&portal_exit_point) == EMPTY {
                            let mut new_path = path.clone();
                            new_path.push(portal_exit_point);
                            paths.push_back(new_path);
                        }
                    }
                }
            }
        }
        None
    }

    fn bfs_with_layers(&self) -> Option<Vec<(i32, i32)>> {
        let mut visited: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
        let mut paths = VecDeque::new();
        paths.push_back((vec![self.start.unwrap().clone()], 0));
        while let Some((path, layer)) = paths.pop_front() {
            if !visited.contains_key(&layer) {
                visited.insert(layer, HashSet::new());
            }
            let current = path.iter().last().unwrap().clone();
            if visited[&layer].contains(&current) {
                continue;
            }
            visited.get_mut(&layer).unwrap().insert(current);
            for target in vec![
                (current.0+1, current.1),
                (current.0-1, current.1),
                (current.0, current.1+1),
                (current.0, current.1-1)
            ] {
                if target == self.end.unwrap() && layer == 0{
                    return Some(path.clone());
                }
                let target_point = self.get_point(&target);
                if target_point == EMPTY {
                    let mut new_path = path.clone();
                    new_path.push(target);
                    paths.push_back((new_path, layer));
                } else if target_point == PORTAL {
                    // skip if blocked
                    if layer < 1 && self.is_edge_portal(&target) {
                        continue;
                    }
                    let portal_exit = *self.portals.get(&target).unwrap();
                    for portal_exit_point in vec![
                        (portal_exit.0+1, portal_exit.1),
                        (portal_exit.0-1, portal_exit.1),
                        (portal_exit.0, portal_exit.1+1),
                        (portal_exit.0, portal_exit.1-1)
                    ] {
                        if self.get_point(&portal_exit_point) == EMPTY {
                            let mut new_path = path.clone();
                            new_path.push(portal_exit_point);
                            if self.is_edge_portal(&target) {
                                paths.push_back((new_path, layer-1));
                            } else {
                                paths.push_back((new_path, layer+1));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn is_edge_portal(&self, portal: &(i32, i32)) -> bool {
        if portal.1 <= self.top_left.1 + 2 {
            // top
            return true
        } else if portal.1 >= self.bottom_right.1 - 2 {
            // bottom 
            return true
        } else if portal.0 <= self.top_left.0 + 2 {
            // left
            return true
        } else if portal.0 >= self.bottom_right.0 -2 {
            // right
            return true
        }
        false
    }

    fn height(&self) -> i32 {
        self.top_left.1.abs() + self.bottom_right.1 + 1
    }

    fn width(&self) -> i32 {
        self.top_left.0.abs() + self.bottom_right.0 + 1
    }
}


pub fn one() {
    let input = get_input();
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter != ' ' {
                map.insert((x as i32, y as i32), letter);
            }
        }
    }
    map.render();
    map.scan_portals();
    map.mark_portals();
    map.render();

    println!("height {}", map.height());
    println!("width {}", map.width());
    let path = map.bfs().unwrap();
    map.render_with_path(&path);
    println!("{:?}", path.len()-2);
}

pub fn two() {
    let input = get_input();
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter != ' ' {
                map.insert((x as i32, y as i32), letter);
            }
        }
    }
    map.render();
    map.scan_portals();
    map.mark_portals();
    map.render();

    println!("height {}", map.height());
    println!("width {}", map.width());
    let start = Instant::now();
    let path = map.bfs_with_layers().unwrap();
    map.render_with_path(&path);
    println!("{:?}", path.len()-2);
    println!("Took {} seconds", start.elapsed().as_secs_f32());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day_20_task_1(){
        let input = get_input();
        let mut map = Map::new();

        for (y, line) in input.lines().enumerate() {
            for (x, letter) in line.chars().enumerate() {
                if letter != ' ' {
                    map.insert((x as i32, y as i32), letter);
                }
            }
        }
        map.scan_portals();
        map.mark_portals();
        let path = map.bfs().unwrap();
        assert_eq!(684, path.len()-2);
    }

    #[test]
    fn day_20_task_2(){
        let input = get_input();
        let mut map = Map::new();

        for (y, line) in input.lines().enumerate() {
            for (x, letter) in line.chars().enumerate() {
                if letter != ' ' {
                    map.insert((x as i32, y as i32), letter);
                }
            }
        }
        map.scan_portals();
        map.mark_portals();
        let path = map.bfs_with_layers().unwrap();
        assert_eq!(7758, path.len()-2);
    }
}