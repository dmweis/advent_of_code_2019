use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};
use std::error;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split(")"))
        .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
        .collect()
}

fn edges_to_directed_graph<'a>(edges: &'a Vec<(&str, &str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph: HashMap<_, Vec<&str>> = HashMap::new();
    for (parent, child) in edges {
        graph.entry(*parent).or_default().push(child);
    }
    graph
}

fn edges_to_undirected_graph<'a>(edges: &'a Vec<(&str, &str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph: HashMap<_, Vec<&str>> = HashMap::new();
    for (a, b) in edges {
        graph.entry(*a).or_default().push(b);
        graph.entry(*b).or_default().push(a);
    }
    graph
}

fn breath_first_search(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> Result<i32, Box<dyn error::Error>> {
    let mut visited = HashSet::new();
    let mut seen = VecDeque::new();
    seen.push_front((start, 0));
    while let Some((node, depth)) = seen.pop_back() {
        // if node was visited already skip
        if !visited.insert(node) {
            continue;
        }
        if node == end {
            return Ok(depth);
        }
        if let Some(relatives) = graph.get(node) {
            for relative in relatives {
                seen.push_front((relative, depth+1));
            }
        }
    }
    Err("End node not found")?
}

fn count_orbits(graph: &HashMap<&str, Vec<&str>>) -> i32 {
    count_orbits_for_key(graph, "COM", -1)
}

fn count_orbits_for_key(graph: &HashMap<&str, Vec<&str>>, node: &str, count: i32) -> i32 {
    let mut counter = count+1;
    if let Some(children) = graph.get(node) {
        for child in children {
            counter += count_orbits_for_key(graph, child, count+1);
        }
    }
    counter
}

pub fn one() {
    let text = read_file("input/day_six.txt");
    let data = parse_input(&text);
    let graph = edges_to_directed_graph(&data);
    let count = count_orbits(&graph);
    println!("{}", count);
}

pub fn two() {
    let text = read_file("input/day_six.txt");
    let parsed = parse_input(&text);
    let graph = edges_to_undirected_graph(&parsed);
    let distance = breath_first_search(&graph, "YOU", "SAN");
    if let Ok(distance) = distance {
        println!("Distance between you and santa {}",distance - 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_input_simple_tuple() {
        let test_input = "A)B\nB)C";
        let parsed = parse_input(test_input);
        let mut parsed_iter = parsed.iter();
        assert_eq!(parsed_iter.next(), Some(&("A", "B")));
        assert_eq!(parsed_iter.next(), Some(&("B", "C")));
    }

    #[test]
    fn orbit_counter_test() {
        let mut graph = HashMap::new();
        graph.insert("COM", vec!["A"]);
        graph.insert("A", vec!["B", "C"]);
        let orbit_count = count_orbits(&graph);
        assert_eq!(orbit_count, 5);
    }

    #[test]
    fn orbit_parse_count_test() {
        let text_input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let parsed = parse_input(text_input);
        let graph = edges_to_directed_graph(&parsed);
        let count = count_orbits(&graph);
        assert_eq!(count, 42);
    }

    #[test]
    fn day_6_task_1() {
        let text = read_file("input/day_six.txt");
        let data = parse_input(&text);
        let graph = edges_to_directed_graph(&data);
        let count = count_orbits(&graph);
        assert_eq!(count, 147807);
    }

    #[test]
    fn breath_search_finds_example() {
        let text_input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let parsed = parse_input(text_input);
        let graph = edges_to_undirected_graph(&parsed);
        let distance = breath_first_search(&graph, "YOU", "SAN").unwrap();
        assert_eq!(distance, 6);
    }

    #[test]
    fn day_6_task_2() {
        let text = read_file("input/day_six.txt");
        let parsed = parse_input(&text);
        let graph = edges_to_undirected_graph(&parsed);
        let distance = breath_first_search(&graph, "YOU", "SAN").unwrap();
        // 229 + 2 because you are adding the orbit of YOU and SAN from the distance
        assert_eq!(distance, 229 + 2);
    }
}
